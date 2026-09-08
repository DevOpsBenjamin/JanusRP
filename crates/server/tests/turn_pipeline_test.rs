use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

use janus_db::{
    create_pool, run_migrations, seed_val_corbeau, SALLE_COMMUNE_LOCATION_ID,
    VAL_CORBEAU_CAMPAIGN_ID,
};
use janus_llm::MockLlmClient;
use janus_server::{create_router, AppState};

async fn setup_test_db() -> Option<janus_db::PgPool> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/janusrp".to_string());

    match create_pool(&db_url).await {
        Ok(pool) => {
            if run_migrations(&pool).await.is_ok() && seed_val_corbeau(&pool).await.is_ok() {
                Some(pool)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[tokio::test]
async fn test_turn_pipeline_full_sse_stream() {
    let campaign_id = Uuid::new_v4();
    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(None, mock_llm.clone());
    let app = create_router(state);

    let req_body = serde_json::json!({
        "player_input": "Je salue poliment Elena la tavernière."
    });

    let request = Request::builder()
        .method("POST")
        .uri(format!("/api/campaigns/{}/turns", campaign_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.contains("text/event-stream"),
        "Content-Type should be text/event-stream, got: {}",
        content_type
    );

    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    // Verify all multiplexed SSE events are present
    assert!(
        body_str.contains("event: turn_start"),
        "Body missing turn_start event: {}",
        body_str
    );
    assert!(
        body_str.contains("event: mj_thinking"),
        "Body missing mj_thinking event: {}",
        body_str
    );
    assert!(
        body_str.contains("arbitrating"),
        "Body missing arbitrating thinking status: {}",
        body_str
    );
    assert!(
        body_str.contains("event: state_mutation"),
        "Body missing state_mutation event: {}",
        body_str
    );
    assert!(
        body_str.contains("relationship_update"),
        "Body missing relationship_update mutation: {}",
        body_str
    );
    assert!(
        body_str.contains("event: narration_chunk"),
        "Body missing narration_chunk event: {}",
        body_str
    );
    assert!(
        body_str.contains("event: turn_complete"),
        "Body missing turn_complete event: {}",
        body_str
    );

    // Verify recorded prompt and briefing in MockLlmClient
    let prompts = mock_llm.get_recorded_prompts();
    assert_eq!(prompts.len(), 1);
    assert_eq!(
        prompts[0].player_input,
        "Je salue poliment Elena la tavernière."
    );

    let briefings = mock_llm.get_recorded_briefings();
    assert_eq!(briefings.len(), 1);
}

#[tokio::test]
async fn test_turn_pipeline_concurrency_conflict_409() {
    let campaign_id = Uuid::new_v4();
    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(None, mock_llm);

    // Pre-acquire campaign lock
    let lock_arc = state.get_campaign_lock(campaign_id).await;
    let _guard = lock_arc.lock().await;

    let app = create_router(state);

    let req_body = serde_json::json!({
        "player_input": "Tentative concurrente d'action"
    });

    let request = Request::builder()
        .method("POST")
        .uri(format!("/api/campaigns/{}/turns", campaign_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(
        body_json["error"]
            .as_str()
            .unwrap()
            .contains("already in progress")
    );
}

#[tokio::test]
async fn test_turn_pipeline_validation_bad_request_400() {
    let campaign_id = Uuid::new_v4();
    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(None, mock_llm);
    let app = create_router(state);

    // Empty player input
    let req_body = serde_json::json!({
        "player_input": "   "
    });

    let request = Request::builder()
        .method("POST")
        .uri(format!("/api/campaigns/{}/turns", campaign_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(
        body_json["error"]
            .as_str()
            .unwrap()
            .contains("must not be empty")
    );
}

#[tokio::test]
async fn test_turn_pipeline_arbitration_error_event() {
    let campaign_id = Uuid::new_v4();
    let mock_llm = Arc::new(MockLlmClient::new().with_arbitration_error("GPU cluster offline"));
    let state = AppState::new(None, mock_llm);
    let app = create_router(state);

    let req_body = serde_json::json!({
        "player_input": "Test d'erreur LLM"
    });

    let request = Request::builder()
        .method("POST")
        .uri(format!("/api/campaigns/{}/turns", campaign_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_str.contains("event: turn_start"));
    assert!(body_str.contains("event: error"));
    assert!(body_str.contains("arbitration_failed"));
    assert!(body_str.contains("GPU cluster offline"));
}

#[tokio::test]
async fn test_turn_pipeline_with_db_persistence() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => {
            eprintln!("PostgreSQL not reachable, skipping live DB test");
            return;
        }
    };

    // Reset Elena relationship and player location
    janus_db::campaigns::update_current_location(
        &pool,
        VAL_CORBEAU_CAMPAIGN_ID,
        Some(SALLE_COMMUNE_LOCATION_ID),
    )
    .await
    .unwrap();

    let campaign_before = janus_db::campaigns::get_by_id(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .expect("Campaign should exist");

    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(Some(pool.clone()), mock_llm);
    let app = create_router(state);

    let req_body = serde_json::json!({
        "player_input": "Je commande une chope d'hydromel et demande des nouvelles."
    });

    let request = Request::builder()
        .method("POST")
        .uri(format!(
            "/api/campaigns/{}/turns",
            VAL_CORBEAU_CAMPAIGN_ID
        ))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_str.contains("event: turn_start"));
    assert!(body_str.contains("event: turn_complete"));

    // Verify DB persistence: turn count incremented
    let campaign_after = janus_db::campaigns::get_by_id(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .expect("Campaign should exist");
    assert_eq!(
        campaign_after.turn_count,
        campaign_before.turn_count + 1
    );

    // Verify turn record created in DB
    let latest_turn = janus_db::turns::get_latest_turn(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .expect("Turn should be stored in DB");
    assert_eq!(
        latest_turn.player_input,
        "Je commande une chope d'hydromel et demande des nouvelles."
    );
    assert!(latest_turn.final_narration.contains("Elena"));

    // Test GET /api/campaigns/:id/turns
    let state2 = AppState::new(Some(pool.clone()), Arc::new(MockLlmClient::new()));
    let app2 = create_router(state2);
    let get_turns_req = Request::builder()
        .method("GET")
        .uri(format!(
            "/api/campaigns/{}/turns",
            VAL_CORBEAU_CAMPAIGN_ID
        ))
        .body(Body::empty())
        .unwrap();
    let get_turns_res = app2.oneshot(get_turns_req).await.unwrap();
    assert_eq!(get_turns_res.status(), StatusCode::OK);

    let turns_bytes = to_bytes(get_turns_res.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let turns_json: serde_json::Value = serde_json::from_slice(&turns_bytes).unwrap();
    assert!(turns_json.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn test_turn_pipeline_campaign_not_found_404_with_db() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => {
            eprintln!("PostgreSQL not reachable, skipping live DB test");
            return;
        }
    };

    let unknown_campaign_id = Uuid::new_v4();
    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(Some(pool), mock_llm);
    let app = create_router(state);

    let req_body = serde_json::json!({
        "player_input": "Test campagne inexistante"
    });

    let request = Request::builder()
        .method("POST")
        .uri(format!("/api/campaigns/{}/turns", unknown_campaign_id))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&req_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
