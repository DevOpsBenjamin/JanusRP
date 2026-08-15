use janus_core::LocationEdge;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct NewLocationEdge {
    pub id: Option<Uuid>,
    pub campaign_id: Uuid,
    pub source_location_id: Uuid,
    pub target_location_id: Uuid,
    pub bidirectional: Option<bool>,
    pub travel_description: Option<String>,
    pub is_locked: Option<bool>,
    pub lock_reason: Option<String>,
    pub metadata: Option<Value>,
}

pub async fn create(pool: &PgPool, new_e: &NewLocationEdge) -> Result<LocationEdge, DbError> {
    let id = new_e.id.unwrap_or_else(Uuid::new_v4);
    let bidirectional = new_e.bidirectional.unwrap_or(true);
    let is_locked = new_e.is_locked.unwrap_or(false);
    let metadata = new_e.metadata.clone().unwrap_or_else(|| serde_json::json!({}));

    let edge = sqlx::query_as::<_, LocationEdge>(
        r#"
        INSERT INTO location_edges (
            id, campaign_id, source_location_id, target_location_id,
            bidirectional, travel_description, is_locked, lock_reason, metadata
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, campaign_id, source_location_id, target_location_id,
                  bidirectional, travel_description, is_locked, lock_reason, metadata, created_at
        "#,
    )
    .bind(id)
    .bind(new_e.campaign_id)
    .bind(new_e.source_location_id)
    .bind(new_e.target_location_id)
    .bind(bidirectional)
    .bind(&new_e.travel_description)
    .bind(is_locked)
    .bind(&new_e.lock_reason)
    .bind(&metadata)
    .fetch_one(pool)
    .await?;

    Ok(edge)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<LocationEdge>, DbError> {
    let edge = sqlx::query_as::<_, LocationEdge>(
        r#"
        SELECT id, campaign_id, source_location_id, target_location_id,
               bidirectional, travel_description, is_locked, lock_reason, metadata, created_at
        FROM location_edges
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(edge)
}

pub async fn list_by_campaign(
    pool: &PgPool,
    campaign_id: Uuid,
) -> Result<Vec<LocationEdge>, DbError> {
    let edges = sqlx::query_as::<_, LocationEdge>(
        r#"
        SELECT id, campaign_id, source_location_id, target_location_id,
               bidirectional, travel_description, is_locked, lock_reason, metadata, created_at
        FROM location_edges
        WHERE campaign_id = $1
        "#,
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await?;

    Ok(edges)
}

pub async fn list_connected_edges(
    pool: &PgPool,
    location_id: Uuid,
) -> Result<Vec<LocationEdge>, DbError> {
    let edges = sqlx::query_as::<_, LocationEdge>(
        r#"
        SELECT id, campaign_id, source_location_id, target_location_id,
               bidirectional, travel_description, is_locked, lock_reason, metadata, created_at
        FROM location_edges
        WHERE source_location_id = $1
           OR (target_location_id = $1 AND bidirectional = TRUE)
        "#,
    )
    .bind(location_id)
    .fetch_all(pool)
    .await?;

    Ok(edges)
}

pub async fn find_edge_between(
    pool: &PgPool,
    source_id: Uuid,
    target_id: Uuid,
) -> Result<Option<LocationEdge>, DbError> {
    let edge = sqlx::query_as::<_, LocationEdge>(
        r#"
        SELECT id, campaign_id, source_location_id, target_location_id,
               bidirectional, travel_description, is_locked, lock_reason, metadata, created_at
        FROM location_edges
        WHERE (source_location_id = $1 AND target_location_id = $2)
           OR (source_location_id = $2 AND target_location_id = $1 AND bidirectional = TRUE)
        "#,
    )
    .bind(source_id)
    .bind(target_id)
    .fetch_optional(pool)
    .await?;

    Ok(edge)
}

pub async fn set_locked(
    pool: &PgPool,
    id: Uuid,
    is_locked: bool,
    lock_reason: Option<String>,
) -> Result<(), DbError> {
    let result = sqlx::query(
        r#"
        UPDATE location_edges
        SET is_locked = $1, lock_reason = $2
        WHERE id = $3
        "#,
    )
    .bind(is_locked)
    .bind(lock_reason)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound {
            table: "location_edges",
            id: id.to_string(),
        });
    }

    Ok(())
}
