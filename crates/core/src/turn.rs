use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
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
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "VARCHAR", rename_all = "snake_case"))]
pub enum EventSignificance {
    Minor,
    Notable,
    Major,
    Critical,
}

impl Default for EventSignificance {
    fn default() -> Self {
        Self::Notable
    }
}

impl std::fmt::Display for EventSignificance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minor => write!(f, "minor"),
            Self::Notable => write!(f, "notable"),
            Self::Major => write!(f, "major"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

impl std::str::FromStr for EventSignificance {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minor" => Ok(Self::Minor),
            "notable" => Ok(Self::Notable),
            "major" => Ok(Self::Major),
            "critical" => Ok(Self::Critical),
            other => Err(format!("Unknown significance: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
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
