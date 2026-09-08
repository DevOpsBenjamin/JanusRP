use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    response::IntoResponse,
    Json,
};
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::orchestrator::{to_sse_event, TurnOrchestrator};
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTurnRequest {
    pub player_input: String,
}

#[derive(Debug, Deserialize)]
pub struct ListTurnsQuery {
    pub limit: Option<i64>,
}

struct CancelOnDrop {
    token: CancellationToken,
}

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.token.cancel();
    }
}

pub async fn create_turn_handler(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    Json(payload): Json<CreateTurnRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, Json<serde_json::Value>)>
{
    // 1. Validate player input
    if payload.player_input.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "player_input must not be empty"
            })),
        ));
    }

    // 2. Validate campaign exists (if DB is active)
    if let Some(ref pool) = state.db {
        let campaign = janus_db::campaigns::get_by_id(pool, campaign_id)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e.to_string() })),
                )
            })?;

        if campaign.is_none() {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Campaign {} not found", campaign_id)
                })),
            ));
        }
    }

    // 3. Acquire per-campaign concurrency lock
    let lock_arc = state.get_campaign_lock(campaign_id).await;
    let lock_guard = match lock_arc.try_lock_owned() {
        Ok(guard) => guard,
        Err(_) => {
            return Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({
                    "error": "A turn is already in progress for this campaign"
                })),
            ));
        }
    };

    // 4. Setup channels and cancellation token
    let (tx, rx) = tokio::sync::mpsc::channel(64);
    let cancel_token = CancellationToken::new();
    let orchestrator = TurnOrchestrator::new(state);

    let task_cancel_token = cancel_token.clone();
    let player_input = payload.player_input;

    // Spawn orchestrator task in background. The lock guard is held until completion.
    tokio::spawn(async move {
        orchestrator
            .run_turn(campaign_id, player_input, tx, task_cancel_token)
            .await;
        drop(lock_guard);
    });

    let cancel_guard = Arc::new(CancelOnDrop {
        token: cancel_token,
    });

    let stream = ReceiverStream::new(rx).map(move |event| {
        let _ = &cancel_guard;
        Ok::<Event, Infallible>(to_sse_event(&event))
    });

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

pub async fn list_turns_handler(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    Query(query): Query<ListTurnsQuery>,
) -> impl IntoResponse {
    let pool = match state.db {
        Some(ref p) => p,
        None => {
            return (
                StatusCode::OK,
                Json(serde_json::json!([])),
            );
        }
    };

    let limit = query.limit.unwrap_or(20);
    match janus_db::turns::list_by_campaign(pool, campaign_id, limit).await {
        Ok(turns) => (StatusCode::OK, Json(serde_json::json!(turns))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}
