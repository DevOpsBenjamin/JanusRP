use janus_core::Location;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct NewLocation {
    pub id: Option<Uuid>,
    pub campaign_id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub atmosphere: Option<String>,
    pub secrets: Option<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub props: Option<Value>,
    pub metadata: Option<Value>,
}

pub async fn create(pool: &PgPool, new_l: &NewLocation) -> Result<Location, DbError> {
    let id = new_l.id.unwrap_or_else(Uuid::new_v4);
    let props = new_l.props.clone().unwrap_or_else(|| serde_json::json!([]));
    let metadata = new_l.metadata.clone().unwrap_or_else(|| serde_json::json!({}));

    let location = sqlx::query_as::<_, Location>(
        r#"
        INSERT INTO locations (
            id, campaign_id, slug, name, description, atmosphere, secrets,
            position_x, position_y, props, metadata
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, campaign_id, slug, name, description, atmosphere, secrets,
                  position_x, position_y, props, metadata, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(new_l.campaign_id)
    .bind(&new_l.slug)
    .bind(&new_l.name)
    .bind(&new_l.description)
    .bind(&new_l.atmosphere)
    .bind(&new_l.secrets)
    .bind(new_l.position_x)
    .bind(new_l.position_y)
    .bind(&props)
    .bind(&metadata)
    .fetch_one(pool)
    .await?;

    Ok(location)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Location>, DbError> {
    let location = sqlx::query_as::<_, Location>(
        r#"
        SELECT id, campaign_id, slug, name, description, atmosphere, secrets,
               position_x, position_y, props, metadata, created_at, updated_at
        FROM locations
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(location)
}

pub async fn get_by_slug(
    pool: &PgPool,
    campaign_id: Uuid,
    slug: &str,
) -> Result<Option<Location>, DbError> {
    let location = sqlx::query_as::<_, Location>(
        r#"
        SELECT id, campaign_id, slug, name, description, atmosphere, secrets,
               position_x, position_y, props, metadata, created_at, updated_at
        FROM locations
        WHERE campaign_id = $1 AND slug = $2
        "#,
    )
    .bind(campaign_id)
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(location)
}

pub async fn list_by_campaign(pool: &PgPool, campaign_id: Uuid) -> Result<Vec<Location>, DbError> {
    let locations = sqlx::query_as::<_, Location>(
        r#"
        SELECT id, campaign_id, slug, name, description, atmosphere, secrets,
               position_x, position_y, props, metadata, created_at, updated_at
        FROM locations
        WHERE campaign_id = $1
        ORDER BY name ASC
        "#,
    )
    .bind(campaign_id)
    .fetch_all(pool)
    .await?;

    Ok(locations)
}

pub async fn update(pool: &PgPool, loc: &Location) -> Result<Location, DbError> {
    let updated = sqlx::query_as::<_, Location>(
        r#"
        UPDATE locations
        SET name = $1, description = $2, atmosphere = $3, secrets = $4,
            position_x = $5, position_y = $6, props = $7, metadata = $8,
            updated_at = NOW()
        WHERE id = $9
        RETURNING id, campaign_id, slug, name, description, atmosphere, secrets,
                  position_x, position_y, props, metadata, created_at, updated_at
        "#,
    )
    .bind(&loc.name)
    .bind(&loc.description)
    .bind(&loc.atmosphere)
    .bind(&loc.secrets)
    .bind(loc.position_x)
    .bind(loc.position_y)
    .bind(&loc.props)
    .bind(&loc.metadata)
    .bind(loc.id)
    .fetch_optional(pool)
    .await?;

    match updated {
        Some(l) => Ok(l),
        None => Err(DbError::NotFound {
            table: "locations",
            id: loc.id.to_string(),
        }),
    }
}
