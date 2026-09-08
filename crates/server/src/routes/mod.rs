pub mod campaigns;
pub mod health;
pub mod turns;

use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/health", get(health::health_check))
        // Campaign routes
        .route("/api/campaigns", get(campaigns::list_campaigns_handler))
        .route("/api/campaigns/:id", get(campaigns::get_campaign_handler))
        // Turn execution (SSE) & list routes
        .route(
            "/api/campaigns/:id/turns",
            post(turns::create_turn_handler).get(turns::list_turns_handler),
        )
        .route(
            "/campaigns/:id/turns",
            post(turns::create_turn_handler).get(turns::list_turns_handler),
        )
        .with_state(state)
}
