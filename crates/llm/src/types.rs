use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnPrompt {
    pub system_prompt: String,
    pub context_summary: String,
    pub player_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MjArbitrationResponse {
    pub reasoning: String,
    pub tool_calls: Vec<ToolCall>,
    pub director_briefing: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorBriefing {
    pub system_prompt: String,
    pub briefing_instructions: String,
    pub context: serde_json::Value,
}
