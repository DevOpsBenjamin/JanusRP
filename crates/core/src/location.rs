use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Location {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub atmosphere: Option<String>,
    pub secrets: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub props: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct LocationEdge {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub source_location_id: Uuid,
    pub target_location_id: Uuid,
    pub bidirectional: bool,
    pub travel_description: Option<String>,
    pub is_locked: bool,
    pub lock_reason: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
