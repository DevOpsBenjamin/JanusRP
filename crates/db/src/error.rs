use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database query failed: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Migration failed: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("Record not found: {table} ({id})")]
    NotFound { table: &'static str, id: String },

    #[error("Constraint violation: {0}")]
    Constraint(String),
}
