use janus_db::PgPool;
use uuid::Uuid;

use crate::error::McpError;
use crate::executor::McpExecutor;
use crate::tools::{get_turn_tools_schema, ToolCall, ToolDefinition, ToolExecutionResult};

#[derive(Debug, Clone)]
pub struct McpServer {
    pool: PgPool,
}

impl McpServer {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn tools_schema(&self) -> Vec<ToolDefinition> {
        get_turn_tools_schema()
    }

    pub async fn execute_tool(
        &self,
        campaign_id: Uuid,
        turn_number: i32,
        tool_call: &ToolCall,
    ) -> Result<ToolExecutionResult, McpError> {
        McpExecutor::execute_tool(&self.pool, campaign_id, turn_number, tool_call).await
    }

    pub async fn execute_tools(
        &self,
        campaign_id: Uuid,
        turn_number: i32,
        tool_calls: &[ToolCall],
    ) -> Result<Vec<ToolExecutionResult>, McpError> {
        McpExecutor::execute_tools(&self.pool, campaign_id, turn_number, tool_calls).await
    }
}
