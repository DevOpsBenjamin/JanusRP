pub mod health;

use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/health", get(health::health_check))
        .with_state(state)
}
