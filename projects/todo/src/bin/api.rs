use std::{
    borrow::Cow,
    env,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use axum::{Router, http::StatusCode, routing::get};
use axum_otel_metrics::HttpMetricsLayerBuilder;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use otel::OtelConfig;
use sentry::integrations::tower::NewSentryLayer;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

use todo::{
    api::{api_config::ApiConfig, api_secret::ApiSecret, app_state::AppState},
    todo::handlers::TODO_TAG,
};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = TODO_TAG),
    ),
    servers(
        (
            url = "/",
        ),
        (
            url = "{url}",
            description = "Custom server (defaults to local)",
            variables(
                ("url" = (default = "http://localhost:3000")),
            ),
        ),
    ),
)]
struct ApiDoc;

fn main() -> anyhow::Result<()> {
    let config = ApiConfig::new()?;

    let release = env::var("IMAGE_TAG")
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| env::var("GIT_COMMIT").ok().filter(|v| !v.is_empty()))
        .map(Cow::Owned);

    let _sentry_guard = sentry::init((
        config.sentry_dsn,
        sentry::ClientOptions {
            release: release,
            ..sentry::ClientOptions::default()
        },
    ));

    let _otel_guard = otel::init_with_transform(
        OtelConfig {
            log_level: config.log_level,
            service_name: config.otel_service_name,
            endpoint: config.otel_endpoint,
            custom_attribute: config.custom_attribute,
            trace_sampling_ratio: config.otel_trace_sampling_ratio,
        },
        |subscriber| subscriber.with(sentry::integrations::tracing::layer()),
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async {
        let shutdown = shutdown::signal::install()?;

        let secret = ApiSecret::new(&config.vault_address, &config.vault_path).await?;

        let options = MySqlConnectOptions::new()
            .host(&config.mysql_host)
            .port(config.mysql_port)
            .username(&config.mysql_username)
            .password(&secret.mysql_password)
            .charset("utf8mb4")
            .collation("utf8mb4_general_ci")
            .database(&config.mysql_database);
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        let is_ready = Arc::new(AtomicBool::new(true));
        let health_check_router = http_util::health_check::router(is_ready.clone());

        let (api_router, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .merge(todo::todo::handlers::router())
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                config.request_timeout,
            ))
            .layer(CorsLayer::very_permissive().max_age(Duration::from_secs(3600)))
            .layer(OtelInResponseLayer::default())
            .layer(OtelAxumLayer::default())
            .layer(NewSentryLayer::new_from_top())
            .layer(HttpMetricsLayerBuilder::new().build())
            .with_state(Arc::new(AppState { db: pool }))
            .split_for_parts();

        let mut app = Router::new()
            .route("/", get(|| async { "Welcome to todo-api!" }))
            .merge(health_check_router)
            .merge(api_router);
        if config.swagger_enabled {
            app = app.merge(SwaggerUi::new("/swagger-ui").url("/docs/openapi.json", openapi));
        }
        app = app.with_state(());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                shutdown.cancelled().await;
                info!("web server shutdown signal received");
                is_ready.store(false, Ordering::Relaxed);
            })
            .await?;

        Ok(())
    })
}
