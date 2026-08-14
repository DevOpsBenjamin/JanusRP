use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "janus-server",
        "version": env!("CARGO_PKG_VERSION"),
        "live_reload": "active",
        "message": "Docker hot reload verified!"
    }))
}
