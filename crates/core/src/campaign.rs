use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Campaign {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub system_prompt_theme: Option<String>,
    pub player_name: String,
    pub current_location_id: Option<Uuid>,
    pub turn_count: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
