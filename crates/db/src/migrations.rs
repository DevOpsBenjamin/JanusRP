use sqlx::PgPool;
use tracing::info;

use crate::error::DbError;

pub async fn run_migrations(pool: &PgPool) -> Result<(), DbError> {
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    info!("Database migrations applied successfully.");
    Ok(())
}
