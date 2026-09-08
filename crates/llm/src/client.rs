use async_trait::async_trait;
use futures_util::Stream;
use std::pin::Pin;

use crate::error::LlmError;
use crate::types::{DirectorBriefing, MjArbitrationResponse, ToolOutput, TurnPrompt};

pub type NarrationStream = Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete_turn_arbitration(
        &self,
        prompt: &TurnPrompt,
    ) -> Result<MjArbitrationResponse, LlmError>;

    async fn continue_turn_arbitration(
        &self,
        _prompt: &TurnPrompt,
        initial_response: &MjArbitrationResponse,
        _tool_outputs: &[ToolOutput],
    ) -> Result<MjArbitrationResponse, LlmError> {
        Ok(initial_response.clone())
    }

    async fn stream_narration(
        &self,
        briefing: &DirectorBriefing,
    ) -> Result<NarrationStream, LlmError>;
}
