use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::state::AppState;

pub async fn list_campaigns_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool = match state.db {
        Some(ref p) => p,
        None => {
            return (
                StatusCode::OK,
                Json(serde_json::json!([])),
            );
        }
    };

    match janus_db::campaigns::list(pool).await {
        Ok(campaigns) => (StatusCode::OK, Json(serde_json::json!(campaigns))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}

pub async fn get_campaign_handler(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = match state.db {
        Some(ref p) => p,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Database not configured" })),
            );
        }
    };

    match janus_db::campaigns::get_by_id(pool, campaign_id).await {
        Ok(Some(campaign)) => {
            let mut location_details = None;
            if let Some(loc_id) = campaign.current_location_id {
                if let Ok(Some(loc)) = janus_db::locations::get_by_id(pool, loc_id).await {
                    location_details = Some(loc);
                }
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "campaign": campaign,
                    "current_location": location_details,
                })),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Campaign {} not found", campaign_id)
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}
