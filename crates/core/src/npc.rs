use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Npc {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub current_location_id: Option<Uuid>,
    pub slug: String,
    pub name: String,
    pub title: Option<String>,
    pub personality_traits: serde_json::Value,
    pub secret_agenda: Option<String>,
    pub background: Option<String>,
    pub is_alive: bool,
    pub is_active: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct NpcRelationship {
    pub id: Uuid,
    pub npc_id: Uuid,
    pub affinity: i32,
    pub trust: i32,
    pub mood: String,
    pub last_interaction_turn: i32,
    pub interaction_summary: Option<String>,
    pub metadata: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

impl NpcRelationship {
    pub fn clamp_gauges(&mut self) {
        self.affinity = self.affinity.clamp(-100, 100);
        self.trust = self.trust.clamp(-100, 100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauge_clamping() {
        let mut rel = NpcRelationship {
            id: Uuid::new_v4(),
            npc_id: Uuid::new_v4(),
            affinity: 150,
            trust: -200,
            mood: "furieuse".to_string(),
            last_interaction_turn: 1,
            interaction_summary: None,
            metadata: serde_json::json!({}),
            updated_at: Utc::now(),
        };

        rel.clamp_gauges();
        assert_eq!(rel.affinity, 100);
        assert_eq!(rel.trust, -100);
    }
}
