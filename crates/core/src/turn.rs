use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub turn_number: i32,
    pub player_input: String,
    pub mj_reasoning: Option<String>,
    pub mj_briefing: Option<String>,
    pub final_narration: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventSignificance {
    Minor,
    Notable,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEvent {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub summary: String,
    pub significance: EventSignificance,
    pub involved_npc_ids: Vec<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}
