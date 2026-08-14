use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error response ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Serialization / Parsing error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid LLM response structure: {0}")]
    InvalidResponse(String),

    #[error("LLM timeout after {0}s")]
    Timeout(u64),
}
