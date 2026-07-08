use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use axum::{Router, extract::State, http::StatusCode, routing::get};

pub async fn liveness_handler() -> StatusCode {
    StatusCode::OK
}

pub async fn readiness_handler(State(is_ready): State<Arc<AtomicBool>>) -> StatusCode {
    if is_ready.load(Ordering::Relaxed) {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

pub fn router(is_ready: Arc<AtomicBool>) -> Router {
    Router::new()
        .route("/alive", get(liveness_handler))
        .route("/ready", get(readiness_handler))
        .with_state(is_ready)
}
