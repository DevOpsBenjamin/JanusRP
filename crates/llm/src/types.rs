use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    pub arguments: serde_json::Value,
}

impl ToolCall {
    pub fn new(name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            id: None,
            name: name.into(),
            arguments,
        }
    }

    pub fn with_id(id: impl Into<String>, name: impl Into<String>, arguments: serde_json::Value) -> Self {
        Self {
            id: Some(id.into()),
            name: name.into(),
            arguments,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolOutput {
    pub call_id: String,
    pub name: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TurnPrompt {
    pub system_prompt: String,
    pub context_summary: String,
    pub player_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MjArbitrationResponse {
    pub reasoning: String,
    pub tool_calls: Vec<ToolCall>,
    pub director_briefing: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectorBriefing {
    pub system_prompt: String,
    pub briefing_instructions: String,
    pub context: serde_json::Value,
    #[serde(default)]
    pub recent_narrations: Vec<String>,
}
