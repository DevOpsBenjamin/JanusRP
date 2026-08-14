use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum StateMutation {
    LocationChange {
        location_id: Uuid,
        location_name: String,
        narration_hint: Option<String>,
    },
    RelationshipUpdate {
        npc_id: Uuid,
        npc_name: String,
        affinity: i32,
        trust: i32,
        mood: String,
        delta_affinity: Option<i32>,
        delta_trust: Option<i32>,
        reason: String,
    },
    EventLogged {
        event_id: Uuid,
        summary: String,
        significance: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum TurnStreamEvent {
    TurnStart {
        turn_id: Uuid,
        campaign_id: Uuid,
        turn_index: i32,
    },
    MjThinking {
        status: String,
        summary: Option<String>,
    },
    StateMutation(StateMutation),
    NarrationChunk {
        chunk: String,
    },
    TurnComplete {
        turn_id: Uuid,
        current_location_id: Option<Uuid>,
        turn_summary: String,
    },
    Error {
        code: String,
        message: String,
        retryable: bool,
    },
}
