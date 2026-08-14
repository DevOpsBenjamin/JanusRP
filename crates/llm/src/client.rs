use async_trait::async_trait;
use futures_util::Stream;
use std::pin::Pin;

use crate::error::LlmError;
use crate::types::{DirectorBriefing, MjArbitrationResponse, TurnPrompt};

pub type NarrationStream = Pin<Box<dyn Stream<Item = Result<String, LlmError>> + Send>>;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete_turn_arbitration(
        &self,
        prompt: &TurnPrompt,
    ) -> Result<MjArbitrationResponse, LlmError>;

    async fn stream_narration(
        &self,
        briefing: &DirectorBriefing,
    ) -> Result<NarrationStream, LlmError>;
}
