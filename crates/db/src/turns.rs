use janus_core::Turn;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct NewTurn {
    pub id: Option<Uuid>,
    pub campaign_id: Uuid,
    pub turn_number: i32,
    pub player_input: String,
    pub mj_reasoning: Option<String>,
    pub mj_briefing: Option<String>,
    pub final_narration: String,
    pub metadata: Option<Value>,
}

pub async fn create(pool: &PgPool, new_t: &NewTurn) -> Result<Turn, DbError> {
    let id = new_t.id.unwrap_or_else(Uuid::new_v4);
    let metadata = new_t.metadata.clone().unwrap_or_else(|| serde_json::json!({}));

    let turn = sqlx::query_as::<_, Turn>(
        r#"
        INSERT INTO turns (
            id, campaign_id, turn_number, player_input, mj_reasoning,
            mj_briefing, final_narration, metadata
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, campaign_id, turn_number, player_input, mj_reasoning,
                  mj_briefing, final_narration, metadata, created_at
        "#,
    )
    .bind(id)
    .bind(new_t.campaign_id)
    .bind(new_t.turn_number)
    .bind(&new_t.player_input)
    .bind(&new_t.mj_reasoning)
    .bind(&new_t.mj_briefing)
    .bind(&new_t.final_narration)
    .bind(&metadata)
    .fetch_one(pool)
    .await?;

    Ok(turn)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Turn>, DbError> {
    let turn = sqlx::query_as::<_, Turn>(
        r#"
        SELECT id, campaign_id, turn_number, player_input, mj_reasoning,
               mj_briefing, final_narration, metadata, created_at
        FROM turns
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(turn)
}

pub async fn get_latest_turn(
    pool: &PgPool,
    campaign_id: Uuid,
) -> Result<Option<Turn>, DbError> {
    let turn = sqlx::query_as::<_, Turn>(
        r#"
        SELECT id, campaign_id, turn_number, player_input, mj_reasoning,
               mj_briefing, final_narration, metadata, created_at
        FROM turns
        WHERE campaign_id = $1
        ORDER BY turn_number DESC
        LIMIT 1
        "#,
    )
    .bind(campaign_id)
    .fetch_optional(pool)
    .await?;

    Ok(turn)
}

pub async fn list_by_campaign(
    pool: &PgPool,
    campaign_id: Uuid,
    limit: i64,
) -> Result<Vec<Turn>, DbError> {
    let turns = sqlx::query_as::<_, Turn>(
        r#"
        SELECT id, campaign_id, turn_number, player_input, mj_reasoning,
               mj_briefing, final_narration, metadata, created_at
        FROM turns
        WHERE campaign_id = $1
        ORDER BY turn_number DESC
        LIMIT $2
        "#,
    )
    .bind(campaign_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(turns)
}
