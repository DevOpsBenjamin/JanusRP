use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Entity not found: {entity} ({id})")]
    NotFound { entity: &'static str, id: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid gauge value: {name}={value} (must be between {min} and {max})")]
    InvalidGauge {
        name: &'static str,
        value: i32,
        min: i32,
        max: i32,
    },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
