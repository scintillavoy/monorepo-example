use opentelemetry::{
    KeyValue, global, propagation::TextMapCompositePropagator, trace::TracerProvider,
};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    logs::SdkLoggerProvider,
    metrics::SdkMeterProvider,
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::{Sampler, SdkTracerProvider},
};
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, Registry, prelude::*, registry::LookupSpan};

use crate::{OtelConfig, guard::OtelGuard};

pub fn init(config: OtelConfig) -> OtelGuard {
    init_with_transform(config, |s| s)
}

// See: https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk/blob/init-tracing-opentelemetry-v0.33.0/init-tracing-opentelemetry/src/config.rs#L500
pub fn init_with_transform<F, S>(config: OtelConfig, transform: F) -> OtelGuard
where
    F: FnOnce(Registry) -> S,
    S: Subscriber + for<'a> LookupSpan<'a> + Send + Sync,
{
    let tracer_provider = init_tracer_provider(&config);
    let meter_provider = init_meter_provider(&config);
    let logger_provider = init_logger_provider(&config);

    let tracer = tracer_provider.tracer("monorepo-example-otel");
    let otel_trace_layer = OpenTelemetryLayer::new(tracer);

    // See: https://github.com/open-telemetry/opentelemetry-rust/blob/62e43c54896b3f759cf6c97ff29870d5ab4db781/examples/logs-basic/src/main.rs
    let otel_log_filter = EnvFilter::builder()
        .with_default_directive(config.log_level.into())
        .from_env_lossy() // This allows RUST_LOG override.
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_log_layer =
        OpenTelemetryTracingBridge::new(&logger_provider).with_filter(otel_log_filter);

    let fmt_filter = EnvFilter::builder()
        .with_default_directive(config.log_level.into())
        .from_env_lossy();
    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(fmt_filter);

    transform(tracing_subscriber::registry())
        .with(otel_trace_layer)
        .with(otel_log_layer)
        .with(fmt_layer)
        .init();

    OtelGuard {
        tracer_provider,
        meter_provider,
        logger_provider,
    }
}

fn build_resource(config: &OtelConfig) -> Resource {
    Resource::builder()
        .with_service_name(config.service_name.clone())
        .with_attributes([KeyValue::new(
            "custom.attribute",
            config.custom_attribute.clone(),
        )])
        .build()
}

fn init_tracer_provider(config: &OtelConfig) -> SdkTracerProvider {
    let baggage_propagator = BaggagePropagator::new();
    let trace_context_propagator = TraceContextPropagator::new();
    let composite_propagator = TextMapCompositePropagator::new(vec![
        Box::new(baggage_propagator),
        Box::new(trace_context_propagator),
    ]);

    global::set_text_map_propagator(composite_propagator);

    let exporter = SpanExporter::builder()
        .with_http()
        .with_endpoint(format!(
            "{}/v1/traces",
            config.endpoint.trim_end_matches('/')
        ))
        .build()
        .unwrap();

    let provider = SdkTracerProvider::builder()
        .with_resource(build_resource(config))
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            config.trace_sampling_ratio,
        ))))
        .with_batch_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());

    provider
}

fn init_meter_provider(config: &OtelConfig) -> SdkMeterProvider {
    let exporter = MetricExporter::builder()
        .with_http()
        .with_endpoint(format!(
            "{}/v1/metrics",
            config.endpoint.trim_end_matches('/')
        ))
        .build()
        .unwrap();

    let provider = SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(build_resource(config))
        .build();

    global::set_meter_provider(provider.clone());

    provider
}

fn init_logger_provider(config: &OtelConfig) -> SdkLoggerProvider {
    let exporter = LogExporter::builder()
        .with_http()
        .with_endpoint(format!("{}/v1/logs", config.endpoint.trim_end_matches('/')))
        .build()
        .unwrap();

    let provider = SdkLoggerProvider::builder()
        .with_resource(build_resource(config))
        .with_batch_exporter(exporter)
        .build();

    provider
}
