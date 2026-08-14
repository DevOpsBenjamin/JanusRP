use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

use janus_llm::MockLlmClient;
use janus_server::{create_router, AppState};

#[tokio::test]
async fn test_health_check_endpoint() {
    let state = AppState::new(None, Arc::new(MockLlmClient::new()));
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
