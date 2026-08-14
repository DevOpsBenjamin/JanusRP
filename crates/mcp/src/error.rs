use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Invalid arguments for tool {tool}: {message}")]
    InvalidArguments { tool: String, message: String },

    #[error("Tool execution failed: {0}")]
    Execution(String),

    #[error("Database error in MCP: {0}")]
    Database(#[from] janus_db::DbError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
