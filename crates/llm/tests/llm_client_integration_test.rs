use axum::{
    extract::Json,
    response::{sse::Event, sse::Sse, IntoResponse, Response},
    routing::post,
    Router,
};
use futures_util::StreamExt;
use janus_llm::{
    DirectorBriefing, HttpLlmClient, HttpLlmConfig, LlmClient, LlmError, MockLlmClient,
    TurnPrompt,
};
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;
use tokio_stream::wrappers::ReceiverStream;

async fn start_mock_llm_server() -> (SocketAddr, tokio::sync::oneshot::Sender<()>) {
    let app = Router::new()
        .route(
            "/v1/glimmer/chat/completions",
            post(handle_glimmer_chat_completion),
        )
        .route(
            "/v1/qwen/chat/completions",
            post(handle_qwen_streaming_completion),
        )
        .route("/v1/error/chat/completions", post(handle_error_completion));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await
            .unwrap();
    });

    (addr, shutdown_tx)
}

async fn handle_glimmer_chat_completion(Json(payload): Json<serde_json::Value>) -> Response {
    // Assert request contains tools and messages
    assert!(payload.get("tools").is_some());
    assert!(payload.get("messages").is_some());

    let response = json!({
        "id": "chatcmpl-glimmer-test",
        "object": "chat.completion",
        "created": 1700000000,
        "model": "meta-muse-glimmer-30b",
        "choices": [
            {
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "<thought>Le joueur salue Elena chaleureusement.</thought>\n<briefing>Elena accueille le voyageur avec un grand sourire.</briefing>",
                    "tool_calls": [
                        {
                            "id": "call_1",
                            "type": "function",
                            "function": {
                                "name": "update_npc_relation",
                                "arguments": "{\"npc_id\":\"00000000-0000-0000-0000-000000000002\",\"delta_affinity\":15,\"mood\":\"amicale\",\"reason\":\"Salutation cordiale\"}"
                            }
                        },
                        {
                            "id": "call_2",
                            "type": "function",
                            "function": {
                                "name": "log_event",
                                "arguments": "{\"summary\":\"Salutation amicale auprès d'Elena\",\"significance\":\"minor\"}"
                            }
                        }
                    ]
                },
                "finish_reason": "tool_calls"
            }
        ]
    });

    Json(response).into_response()
}

async fn handle_qwen_streaming_completion(Json(payload): Json<serde_json::Value>) -> Response {
    assert_eq!(payload.get("stream"), Some(&json!(true)));

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, std::convert::Infallible>>(16);

    tokio::spawn(async move {
        let chunks = vec![
            "<narrative>\nLa taverne embaume le pin et l'hydromel.\n</narrative>\n\n",
            "<dialogue speaker=\"Elena\" mood=\"warm\" tone=\"friendly\">\n",
            "« Installez-vous près du foyer ! »\n",
            "</dialogue>",
        ];

        for chunk in chunks {
            let data = json!({
                "id": "chatcmpl-qwen-test",
                "object": "chat.completion.chunk",
                "created": 1700000000,
                "model": "qwen-3.8",
                "choices": [
                    {
                        "index": 0,
                        "delta": {
                            "content": chunk
                        },
                        "finish_reason": null
                    }
                ]
            });

            let event = Event::default().data(data.to_string());
            if tx.send(Ok(event)).await.is_err() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        let done_event = Event::default().data("[DONE]");
        let _ = tx.send(Ok(done_event)).await;
    });

    Sse::new(ReceiverStream::new(rx)).into_response()
}

async fn handle_error_completion() -> Response {
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Internal LLM cluster fault",
    )
        .into_response()
}

#[tokio::test]
async fn test_http_llm_client_arbitration_and_streaming() {
    let (addr, _shutdown) = start_mock_llm_server().await;

    let config = HttpLlmConfig::default()
        .with_glimmer(
            format!("http://{}/v1/glimmer", addr),
            "meta-muse-glimmer-30b",
        )
        .with_qwen(format!("http://{}/v1/qwen", addr), "qwen-3.8")
        .with_timeout(10);

    let client = HttpLlmClient::new(config).expect("Failed to build HttpLlmClient");

    // 1. Test arbitration with tool calls
    let prompt = TurnPrompt {
        system_prompt: "MJ system rules".to_string(),
        context_summary: "Auberge de Val-Corbeau".to_string(),
        player_input: "Je salue poliment la tavernière".to_string(),
    };

    let arb = client
        .complete_turn_arbitration(&prompt)
        .await
        .expect("Arbitration failed");

    assert_eq!(arb.reasoning, "Le joueur salue Elena chaleureusement.");
    assert_eq!(
        arb.director_briefing,
        "Elena accueille le voyageur avec un grand sourire."
    );
    assert_eq!(arb.tool_calls.len(), 2);
    assert_eq!(arb.tool_calls[0].name, "update_npc_relation");
    assert_eq!(
        arb.tool_calls[0].arguments["delta_affinity"],
        json!(15)
    );
    assert_eq!(arb.tool_calls[1].name, "log_event");

    // 2. Test streaming narration
    let briefing = DirectorBriefing {
        system_prompt: "Plume system instructions".to_string(),
        briefing_instructions: arb.director_briefing,
        context: json!({ "npc": "Elena" }),
    };

    let mut stream = client
        .stream_narration(&briefing)
        .await
        .expect("Streaming failed");

    let mut accumulated = String::new();
    let mut chunk_count = 0;
    while let Some(chunk_res) = stream.next().await {
        let chunk = chunk_res.expect("Error in stream chunk");
        accumulated.push_str(&chunk);
        chunk_count += 1;
    }

    assert_eq!(chunk_count, 4);
    assert!(accumulated.contains("<narrative>"));
    assert!(accumulated.contains("« Installez-vous près du foyer ! »"));
    assert!(accumulated.contains("</dialogue>"));
}

#[tokio::test]
async fn test_http_llm_client_api_error() {
    let (addr, _shutdown) = start_mock_llm_server().await;

    let config = HttpLlmConfig::default()
        .with_glimmer(format!("http://{}/v1/error", addr), "broken-model")
        .with_qwen(format!("http://{}/v1/error", addr), "broken-model")
        .with_timeout(5);

    let client = HttpLlmClient::new(config).unwrap();

    let prompt = TurnPrompt {
        system_prompt: "MJ".to_string(),
        context_summary: "Ctx".to_string(),
        player_input: "Input".to_string(),
    };

    let res = client.complete_turn_arbitration(&prompt).await;
    match res {
        Err(LlmError::Api { status, message }) => {
            assert_eq!(status, 500);
            assert!(message.contains("Internal LLM cluster fault"));
        }
        other => panic!("Expected Api error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_mock_llm_client_determinism_and_recording() {
    let mock = MockLlmClient::new();

    let prompt = TurnPrompt {
        system_prompt: "MJ test".to_string(),
        context_summary: "Auberge".to_string(),
        player_input: "Bonjour".to_string(),
    };

    let res = mock.complete_turn_arbitration(&prompt).await.unwrap();
    assert_eq!(res.tool_calls.len(), 2);
    assert_eq!(mock.get_recorded_prompts().len(), 1);

    let briefing = DirectorBriefing {
        system_prompt: "Plume".to_string(),
        briefing_instructions: res.director_briefing,
        context: json!({}),
    };

    let mut stream = mock.stream_narration(&briefing).await.unwrap();
    let mut total = String::new();
    while let Some(chunk) = stream.next().await {
        total.push_str(&chunk.unwrap());
    }

    assert!(total.contains("Elena"));
    assert_eq!(mock.get_recorded_briefings().len(), 1);
}
