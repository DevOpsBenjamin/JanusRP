use janus_core::Campaign;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct NewCampaign {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub system_prompt_theme: Option<String>,
    pub player_name: Option<String>,
    pub metadata: Option<Value>,
}

pub async fn create(pool: &PgPool, new_c: &NewCampaign) -> Result<Campaign, DbError> {
    let id = new_c.id.unwrap_or_else(Uuid::new_v4);
    let player_name = new_c
        .player_name
        .clone()
        .unwrap_or_else(|| "Aventurier".to_string());
    let metadata = new_c.metadata.clone().unwrap_or_else(|| serde_json::json!({}));

    let campaign = sqlx::query_as::<_, Campaign>(
        r#"
        INSERT INTO campaigns (
            id, title, description, system_prompt_theme, player_name, turn_count, metadata
        ) VALUES ($1, $2, $3, $4, $5, 0, $6)
        RETURNING id, title, description, system_prompt_theme, player_name, current_location_id, turn_count, metadata, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&new_c.title)
    .bind(&new_c.description)
    .bind(&new_c.system_prompt_theme)
    .bind(&player_name)
    .bind(&metadata)
    .fetch_one(pool)
    .await?;

    Ok(campaign)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Campaign>, DbError> {
    let campaign = sqlx::query_as::<_, Campaign>(
        r#"
        SELECT id, title, description, system_prompt_theme, player_name, current_location_id, turn_count, metadata, created_at, updated_at
        FROM campaigns
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(campaign)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Campaign>, DbError> {
    let campaigns = sqlx::query_as::<_, Campaign>(
        r#"
        SELECT id, title, description, system_prompt_theme, player_name, current_location_id, turn_count, metadata, created_at, updated_at
        FROM campaigns
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(campaigns)
}

pub async fn update_current_location(
    pool: &PgPool,
    id: Uuid,
    location_id: Option<Uuid>,
) -> Result<(), DbError> {
    let result = sqlx::query(
        r#"
        UPDATE campaigns
        SET current_location_id = $1, updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(location_id)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound {
            table: "campaigns",
            id: id.to_string(),
        });
    }

    Ok(())
}

pub async fn increment_turn_count(pool: &PgPool, id: Uuid) -> Result<i32, DbError> {
    let row = sqlx::query_scalar::<_, i32>(
        r#"
        UPDATE campaigns
        SET turn_count = turn_count + 1, updated_at = NOW()
        WHERE id = $1
        RETURNING turn_count
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(count) => Ok(count),
        None => Err(DbError::NotFound {
            table: "campaigns",
            id: id.to_string(),
        }),
    }
}

pub async fn update_metadata(pool: &PgPool, id: Uuid, metadata: &Value) -> Result<(), DbError> {
    let result = sqlx::query(
        r#"
        UPDATE campaigns
        SET metadata = $1, updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(metadata)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound {
            table: "campaigns",
            id: id.to_string(),
        });
    }

    Ok(())
}
