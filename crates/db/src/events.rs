use janus_core::{EventSignificance, NarrativeEvent};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct NewNarrativeEvent {
    pub id: Option<Uuid>,
    pub campaign_id: Uuid,
    pub turn_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub summary: String,
    pub significance: Option<EventSignificance>,
    pub involved_npc_ids: Option<Vec<Uuid>>,
    pub tags: Option<Vec<String>>,
}

pub async fn create(pool: &PgPool, new_e: &NewNarrativeEvent) -> Result<NarrativeEvent, DbError> {
    let id = new_e.id.unwrap_or_else(Uuid::new_v4);
    let significance = new_e.significance.clone().unwrap_or(EventSignificance::Notable);
    let involved_npc_ids = new_e.involved_npc_ids.clone().unwrap_or_default();
    let tags = new_e.tags.clone().unwrap_or_default();

    let event = sqlx::query_as::<_, NarrativeEvent>(
        r#"
        INSERT INTO narrative_events (
            id, campaign_id, turn_id, location_id, summary,
            significance, involved_npc_ids, tags
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, campaign_id, turn_id, location_id, summary,
                  significance, involved_npc_ids, tags, created_at
        "#,
    )
    .bind(id)
    .bind(new_e.campaign_id)
    .bind(new_e.turn_id)
    .bind(new_e.location_id)
    .bind(&new_e.summary)
    .bind(significance.to_string())
    .bind(&involved_npc_ids)
    .bind(&tags)
    .fetch_one(pool)
    .await?;

    Ok(event)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<NarrativeEvent>, DbError> {
    let event = sqlx::query_as::<_, NarrativeEvent>(
        r#"
        SELECT id, campaign_id, turn_id, location_id, summary,
               significance, involved_npc_ids, tags, created_at
        FROM narrative_events
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(event)
}

pub async fn list_by_campaign(
    pool: &PgPool,
    campaign_id: Uuid,
    limit: i64,
) -> Result<Vec<NarrativeEvent>, DbError> {
    let events = sqlx::query_as::<_, NarrativeEvent>(
        r#"
        SELECT id, campaign_id, turn_id, location_id, summary,
               significance, involved_npc_ids, tags, created_at
        FROM narrative_events
        WHERE campaign_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(campaign_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(events)
}

pub async fn list_by_turn(
    pool: &PgPool,
    turn_id: Uuid,
) -> Result<Vec<NarrativeEvent>, DbError> {
    let events = sqlx::query_as::<_, NarrativeEvent>(
        r#"
        SELECT id, campaign_id, turn_id, location_id, summary,
               significance, involved_npc_ids, tags, created_at
        FROM narrative_events
        WHERE turn_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(turn_id)
    .fetch_all(pool)
    .await?;

    Ok(events)
}
