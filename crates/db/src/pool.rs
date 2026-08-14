use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

use crate::error::DbError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, DbError> {
    info!("Connecting to PostgreSQL database...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await?;

    info!("Connected to PostgreSQL successfully");
    Ok(pool)
}
