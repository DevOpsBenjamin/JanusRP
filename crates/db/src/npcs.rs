use janus_core::{Npc, NpcRelationship};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct NewNpc {
    pub id: Option<Uuid>,
    pub campaign_id: Uuid,
    pub current_location_id: Option<Uuid>,
    pub slug: String,
    pub name: String,
    pub title: Option<String>,
    pub personality_traits: Option<Value>,
    pub secret_agenda: Option<String>,
    pub background: Option<String>,
    pub metadata: Option<Value>,
}

pub async fn create(pool: &PgPool, new_npc: &NewNpc) -> Result<Npc, DbError> {
    let id = new_npc.id.unwrap_or_else(Uuid::new_v4);
    let traits = new_npc
        .personality_traits
        .clone()
        .unwrap_or_else(|| serde_json::json!([]));
    let metadata = new_npc
        .metadata
        .clone()
        .unwrap_or_else(|| serde_json::json!({}));

    let npc = sqlx::query_as::<_, Npc>(
        r#"
        INSERT INTO npcs (
            id, campaign_id, current_location_id, slug, name, title,
            personality_traits, secret_agenda, background, is_alive, is_active, metadata
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, true, true, $10)
        RETURNING id, campaign_id, current_location_id, slug, name, title,
                  personality_traits, secret_agenda, background, is_alive, is_active,
                  metadata, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(new_npc.campaign_id)
    .bind(new_npc.current_location_id)
    .bind(&new_npc.slug)
    .bind(&new_npc.name)
    .bind(&new_npc.title)
    .bind(&traits)
    .bind(&new_npc.secret_agenda)
    .bind(&new_npc.background)
    .bind(&metadata)
    .fetch_one(pool)
    .await?;

    Ok(npc)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Npc>, DbError> {
    let npc = sqlx::query_as::<_, Npc>(
        r#"
        SELECT id, campaign_id, current_location_id, slug, name, title,
               personality_traits, secret_agenda, background, is_alive, is_active,
               metadata, created_at, updated_at
        FROM npcs
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(npc)
}

pub async fn get_by_slug(
    pool: &PgPool,
    campaign_id: Uuid,
    slug: &str,
) -> Result<Option<Npc>, DbError> {
    let npc = sqlx::query_as::<_, Npc>(
        r#"
        SELECT id, campaign_id, current_location_id, slug, name, title,
               personality_traits, secret_agenda, background, is_alive, is_active,
               metadata, created_at, updated_at
        FROM npcs
        WHERE campaign_id = $1 AND slug = $2
        "#,
    )
    .bind(campaign_id)
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(npc)
}

pub async fn list_by_campaign(pool: &PgPool, campaign_id: Uuid) -> Result<Vec<Npc>, DbError> {
    let npcs = sqlx::query_as::<_, Npc>(
        r#"
        SELECT id, campaign_id, current_location_id, slug, name, title,
               personality_traits, secret_agenda, background, is_alive, is_active,
               metadata, created_at, updated_at
        FROM npcs
        WHERE campaign_id = $1
        ORDER BY name ASC
        "#,
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await?;

    Ok(npcs)
}

pub async fn list_by_location(pool: &PgPool, location_id: Uuid) -> Result<Vec<Npc>, DbError> {
    let npcs = sqlx::query_as::<_, Npc>(
        r#"
        SELECT id, campaign_id, current_location_id, slug, name, title,
               personality_traits, secret_agenda, background, is_alive, is_active,
               metadata, created_at, updated_at
        FROM npcs
        WHERE current_location_id = $1 AND is_alive = true
        ORDER BY name ASC
        "#,
    )
    .bind(location_id)
    .fetch_all(pool)
    .await?;

    Ok(npcs)
}

pub async fn update_location(
    pool: &PgPool,
    id: Uuid,
    location_id: Option<Uuid>,
) -> Result<(), DbError> {
    let result = sqlx::query(
        r#"
        UPDATE npcs
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
            table: "npcs",
            id: id.to_string(),
        });
    }

    Ok(())
}

pub async fn create_relationship(
    pool: &PgPool,
    npc_id: Uuid,
    affinity: i32,
    trust: i32,
    mood: &str,
    interaction_summary: Option<&str>,
) -> Result<NpcRelationship, DbError> {
    let id = Uuid::new_v4();
    let clamped_affinity = affinity.clamp(-100, 100);
    let clamped_trust = trust.clamp(-100, 100);

    let rel = sqlx::query_as::<_, NpcRelationship>(
        r#"
        INSERT INTO npc_relationships (
            id, npc_id, affinity, trust, mood, last_interaction_turn,
            interaction_summary, metadata
        ) VALUES ($1, $2, $3, $4, $5, 0, $6, '{}'::jsonb)
        RETURNING id, npc_id, affinity, trust, mood, last_interaction_turn,
                  interaction_summary, metadata, updated_at
        "#,
    )
    .bind(id)
    .bind(npc_id)
    .bind(clamped_affinity)
    .bind(clamped_trust)
    .bind(mood)
    .bind(interaction_summary)
    .fetch_one(pool)
    .await?;

    Ok(rel)
}

pub async fn get_relationship(
    pool: &PgPool,
    npc_id: Uuid,
) -> Result<Option<NpcRelationship>, DbError> {
    let rel = sqlx::query_as::<_, NpcRelationship>(
        r#"
        SELECT id, npc_id, affinity, trust, mood, last_interaction_turn,
               interaction_summary, metadata, updated_at
        FROM npc_relationships
        WHERE npc_id = $1
        "#,
    )
    .bind(npc_id)
    .fetch_optional(pool)
    .await?;

    Ok(rel)
}

pub async fn update_relationship_deltas(
    pool: &PgPool,
    npc_id: Uuid,
    delta_affinity: i32,
    delta_trust: i32,
    mood: Option<&str>,
    interaction_summary: Option<&str>,
    turn: i32,
) -> Result<NpcRelationship, DbError> {
    let current = get_relationship(pool, npc_id).await?;

    let (current_affinity, current_trust, current_mood, current_summary) = match &current {
        Some(rel) => (
            rel.affinity,
            rel.trust,
            rel.mood.as_str(),
            rel.interaction_summary.as_deref(),
        ),
        None => (0, 0, "neutre", None),
    };

    let new_affinity = (current_affinity + delta_affinity).clamp(-100, 100);
    let new_trust = (current_trust + delta_trust).clamp(-100, 100);
    let new_mood = mood.unwrap_or(current_mood);
    let new_summary = interaction_summary.or(current_summary);

    if current.is_none() {
        create_relationship(
            pool,
            npc_id,
            new_affinity,
            new_trust,
            new_mood,
            new_summary,
        )
        .await
    } else {
        let updated = sqlx::query_as::<_, NpcRelationship>(
            r#"
            UPDATE npc_relationships
            SET affinity = $1,
                trust = $2,
                mood = $3,
                last_interaction_turn = $4,
                interaction_summary = $5,
                updated_at = NOW()
            WHERE npc_id = $6
            RETURNING id, npc_id, affinity, trust, mood, last_interaction_turn,
                      interaction_summary, metadata, updated_at
            "#,
        )
        .bind(new_affinity)
        .bind(new_trust)
        .bind(new_mood)
        .bind(turn)
        .bind(new_summary)
        .bind(npc_id)
        .fetch_one(pool)
        .await?;

        Ok(updated)
    }
}
