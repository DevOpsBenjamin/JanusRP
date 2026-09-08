use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

use janus_db::{
    create_pool, run_migrations, seed_val_corbeau, ARRIERE_COUR_LOCATION_ID,
    SALLE_COMMUNE_LOCATION_ID, VAL_CORBEAU_CAMPAIGN_ID,
};
use janus_llm::types::{MjArbitrationResponse, ToolCall};
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
async fn test_e2e_playable_core_loop_demonstration() {
    let pool = match setup_test_db().await {
        Some(p) => p,
        None => {
            eprintln!("PostgreSQL not reachable, running mock-level E2E demonstration");
            return run_e2e_in_memory_demonstration().await;
        }
    };

    // 1. Reset campaign to initial seed state
    janus_db::campaigns::update_current_location(
        &pool,
        VAL_CORBEAU_CAMPAIGN_ID,
        Some(SALLE_COMMUNE_LOCATION_ID),
    )
    .await
    .unwrap();

    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(Some(pool.clone()), mock_llm.clone());
    let app = create_router(state);

    // 2. Verify healthcheck endpoint
    let health_req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let health_res = app.clone().oneshot(health_req).await.unwrap();
    assert_eq!(health_res.status(), StatusCode::OK);

    // 3. Verify starter campaign endpoint
    let camp_req = Request::builder()
        .uri(format!("/api/campaigns/{}", VAL_CORBEAU_CAMPAIGN_ID))
        .body(Body::empty())
        .unwrap();
    let camp_res = app.clone().oneshot(camp_req).await.unwrap();
    assert_eq!(camp_res.status(), StatusCode::OK);

    let camp_bytes = to_bytes(camp_res.into_body(), 1024 * 1024).await.unwrap();
    let camp_json: serde_json::Value = serde_json::from_slice(&camp_bytes).unwrap();
    assert_eq!(camp_json["campaign"]["title"], "Les Brumes de Val-Corbeau");
    assert_eq!(
        camp_json["current_location"]["id"],
        SALLE_COMMUNE_LOCATION_ID.to_string()
    );

    // 4. Playable Turn 1: Social interaction in the tavern
    let turn1_req_body = serde_json::json!({
        "player_input": "Je m'approche du comptoir, salue chaleureusement Elena et commande une chope d'hydromel."
    });

    let turn1_req = Request::builder()
        .method("POST")
        .uri(format!(
            "/api/campaigns/{}/turns",
            VAL_CORBEAU_CAMPAIGN_ID
        ))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&turn1_req_body).unwrap()))
        .unwrap();

    let turn1_res = app.clone().oneshot(turn1_req).await.unwrap();
    assert_eq!(turn1_res.status(), StatusCode::OK);
    assert!(turn1_res
        .headers()
        .get(header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .contains("text/event-stream"));

    let turn1_bytes = to_bytes(turn1_res.into_body(), 1024 * 1024).await.unwrap();
    let turn1_stream = String::from_utf8(turn1_bytes.to_vec()).unwrap();

    // Verify multiplexed SSE events for Turn 1
    assert!(turn1_stream.contains("event: turn_start"));
    assert!(turn1_stream.contains("event: mj_thinking"));
    assert!(turn1_stream.contains("event: state_mutation"));
    assert!(turn1_stream.contains("relationship_update"));
    assert!(turn1_stream.contains("Elena"));
    assert!(turn1_stream.contains("event: narration_chunk"));
    assert!(turn1_stream.contains("<narrative>"));
    assert!(turn1_stream.contains("<dialogue"));
    assert!(turn1_stream.contains("event: turn_complete"));

    // 5. Verify persistence of Turn 1 in DB
    let latest_turn = janus_db::turns::get_latest_turn(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .expect("Turn 1 must be persisted in SQLx");
    assert_eq!(
        latest_turn.player_input,
        "Je m'approche du comptoir, salue chaleureusement Elena et commande une chope d'hydromel."
    );

    // 6. Playable Turn 2: Spatial movement to L'Arrière-Cour
    let movement_arbitration = MjArbitrationResponse {
        reasoning: "Le joueur franchit la porte battante et sort dans l'arrière-cour.".to_string(),
        tool_calls: vec![ToolCall {
            name: "move_to_location".to_string(),
            arguments: serde_json::json!({
                "target_location_id": ARRIERE_COUR_LOCATION_ID,
                "movement_narration_hint": "La porte battante grince alors que le joueur s'engouffre dans la brume nocturne.",
            }),
        }],
        director_briefing: "Décris la transition vers la cour boueuse, l'odeur d'humidité et la silhouette de Gaston.".to_string(),
    };

    let mock_llm_turn2 = Arc::new(
        MockLlmClient::new()
            .with_arbitration(movement_arbitration)
            .with_narration(vec![
                "<narrative>\nLe froid saisissant de la nuit vous enveloppe alors que vous quittez la chaleur de l'âtre.\n</narrative>\n".to_string(),
                "<sensory type=\"smell\">\nUne odeur de terre détrempée et de bois pourri emplit la cour.\n</sensory>\n".to_string(),
            ]),
    );

    let state_turn2 = AppState::new(Some(pool.clone()), mock_llm_turn2);
    let app_turn2 = create_router(state_turn2);

    let turn2_req_body = serde_json::json!({
        "player_input": "Je pousse la porte battante et je me glisse discrètement dans l'arrière-cour."
    });

    let turn2_req = Request::builder()
        .method("POST")
        .uri(format!(
            "/api/campaigns/{}/turns",
            VAL_CORBEAU_CAMPAIGN_ID
        ))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&turn2_req_body).unwrap()))
        .unwrap();

    let turn2_res = app_turn2.clone().oneshot(turn2_req).await.unwrap();
    assert_eq!(turn2_res.status(), StatusCode::OK);

    let turn2_bytes = to_bytes(turn2_res.into_body(), 1024 * 1024).await.unwrap();
    let turn2_stream = String::from_utf8(turn2_bytes.to_vec()).unwrap();

    // Verify movement mutation in SSE stream
    assert!(turn2_stream.contains("event: state_mutation"));
    assert!(turn2_stream.contains("location_change"));
    assert!(turn2_stream.contains(&ARRIERE_COUR_LOCATION_ID.to_string()));
    assert!(turn2_stream.contains("<sensory"));
    assert!(turn2_stream.contains("event: turn_complete"));

    // 7. Verify updated campaign location in DB
    let camp_after = janus_db::campaigns::get_by_id(&pool, VAL_CORBEAU_CAMPAIGN_ID)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        camp_after.current_location_id,
        Some(ARRIERE_COUR_LOCATION_ID),
        "Player position must be updated to Arrière-Cour"
    );

    // 8. Verify turns list returns both turns
    let turns_req = Request::builder()
        .uri(format!(
            "/api/campaigns/{}/turns",
            VAL_CORBEAU_CAMPAIGN_ID
        ))
        .body(Body::empty())
        .unwrap();
    let turns_res = app_turn2.oneshot(turns_req).await.unwrap();
    assert_eq!(turns_res.status(), StatusCode::OK);

    let turns_bytes = to_bytes(turns_res.into_body(), 1024 * 1024).await.unwrap();
    let turns_list: Vec<serde_json::Value> = serde_json::from_slice(&turns_bytes).unwrap();
    assert!(turns_list.len() >= 2);
}

async fn run_e2e_in_memory_demonstration() {
    let mock_llm = Arc::new(MockLlmClient::new());
    let state = AppState::new(None, mock_llm);
    let app = create_router(state);

    let turn_req_body = serde_json::json!({
        "player_input": "Test d'action dans l'auberge"
    });

    let turn_req = Request::builder()
        .method("POST")
        .uri(format!("/api/campaigns/{}/turns", uuid::Uuid::new_v4()))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&turn_req_body).unwrap()))
        .unwrap();

    let res = app.oneshot(turn_req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), 1024 * 1024).await.unwrap();
    let stream_str = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(stream_str.contains("event: turn_start"));
    assert!(stream_str.contains("event: turn_complete"));
}
