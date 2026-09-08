use axum::response::sse::Event;
use janus_core::TurnStreamEvent;

pub fn to_sse_event(event: &TurnStreamEvent) -> Event {
    let event_name = event.event_type();
    let data_value = match event {
        TurnStreamEvent::TurnStart {
            turn_id,
            campaign_id,
            turn_index,
        } => serde_json::json!({
            "event": "turn_start",
            "turn_id": turn_id,
            "campaign_id": campaign_id,
            "turn_index": turn_index,
        }),
        TurnStreamEvent::MjThinking { status, summary } => serde_json::json!({
            "event": "mj_thinking",
            "status": status,
            "summary": summary,
        }),
        TurnStreamEvent::StateMutation(mutation) => {
            let mut val = serde_json::to_value(mutation).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = val.as_object_mut() {
                obj.insert(
                    "event".to_string(),
                    serde_json::Value::String("state_mutation".to_string()),
                );
            }
            val
        }
        TurnStreamEvent::NarrationChunk { chunk } => serde_json::json!({
            "event": "narration_chunk",
            "chunk": chunk,
        }),
        TurnStreamEvent::TurnComplete {
            turn_id,
            current_location_id,
            turn_summary,
            mj_reasoning,
            mj_briefing,
            tool_calls,
        } => serde_json::json!({
            "event": "turn_complete",
            "turn_id": turn_id,
            "current_location_id": current_location_id,
            "current_location": current_location_id,
            "turn_summary": turn_summary,
            "mj_reasoning": mj_reasoning,
            "mj_briefing": mj_briefing,
            "tool_calls": tool_calls,
        }),
        TurnStreamEvent::Error {
            code,
            message,
            retryable,
        } => serde_json::json!({
            "event": "error",
            "code": code,
            "message": message,
            "retryable": retryable,
        }),
    };

    Event::default()
        .event(event_name)
        .data(serde_json::to_string(&data_value).unwrap_or_default())
}
