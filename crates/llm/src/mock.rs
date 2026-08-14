use async_trait::async_trait;
use tokio_stream::iter;

use crate::client::{LlmClient, NarrationStream};
use crate::error::LlmError;
use crate::types::{DirectorBriefing, MjArbitrationResponse, ToolCall, TurnPrompt};

#[derive(Debug, Clone, Default)]
pub struct MockLlmClient {
    pub custom_arbitration: Option<MjArbitrationResponse>,
    pub custom_narration_chunks: Option<Vec<String>>,
}

impl MockLlmClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_arbitration(mut self, resp: MjArbitrationResponse) -> Self {
        self.custom_arbitration = Some(resp);
        self
    }

    pub fn with_narration(mut self, chunks: Vec<String>) -> Self {
        self.custom_narration_chunks = Some(chunks);
        self
    }
}

#[async_trait]
impl LlmClient for MockLlmClient {
    async fn complete_turn_arbitration(
        &self,
        _prompt: &TurnPrompt,
    ) -> Result<MjArbitrationResponse, LlmError> {
        if let Some(ref custom) = self.custom_arbitration {
            return Ok(custom.clone());
        }

        // Default mock response: update affinity with Elena and brief Qwen
        Ok(MjArbitrationResponse {
            reasoning: "Le joueur a salué Elena avec respect. Elle apprécie la politesse.".to_string(),
            tool_calls: vec![
                ToolCall {
                    name: "update_npc_relation".to_string(),
                    arguments: serde_json::json!({
                        "npc_id": "00000000-0000-0000-0000-000000000002",
                        "delta_affinity": 10,
                        "mood": "souriante",
                        "reason": "Salutation respectueuse et chaleureuse"
                    }),
                },
                ToolCall {
                    name: "log_event".to_string(),
                    arguments: serde_json::json!({
                        "summary": "Le joueur a brisé la glace avec Elena la tavernière.",
                        "significance": "minor"
                    }),
                },
            ],
            director_briefing: "Elena esquisse un sourire et essuie le comptoir en bois sombre. Rédige une description sensorielle de la taverne et son dialogue accueillant.".to_string(),
        })
    }

    async fn stream_narration(
        &self,
        _briefing: &DirectorBriefing,
    ) -> Result<NarrationStream, LlmError> {
        let chunks = self.custom_narration_chunks.clone().unwrap_or_else(|| {
            vec![
                "<narrative>\nLa lueur des braises projette des ombres dansantes sur le plancher ciré de l'auberge.\n</narrative>\n\n".to_string(),
                "<dialogue speaker=\"Elena\" mood=\"warm\" tone=\"welcoming\">\n".to_string(),
                "« Bienvenue aux Brumes de Val-Corbeau, voyageur. Qu'est-ce qui vous amène par une nuit pareille ? »\n".to_string(),
                "</dialogue>".to_string(),
            ]
        });

        let stream = iter(chunks.into_iter().map(Ok));
        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_mock_llm_client() {
        let client = MockLlmClient::new();
        let prompt = TurnPrompt {
            system_prompt: "MJ System".to_string(),
            context_summary: "Auberge".to_string(),
            player_input: "Bonjour".to_string(),
        };

        let res = client.complete_turn_arbitration(&prompt).await.unwrap();
        assert!(!res.tool_calls.is_empty());
        assert!(!res.director_briefing.is_empty());

        let briefing = DirectorBriefing {
            system_prompt: "Plume System".to_string(),
            briefing_instructions: res.director_briefing,
            context: serde_json::json!({}),
        };

        let mut stream = client.stream_narration(&briefing).await.unwrap();
        let mut full_text = String::new();
        while let Some(chunk) = stream.next().await {
            full_text.push_str(&chunk.unwrap());
        }

        assert!(full_text.contains("<dialogue"));
        assert!(full_text.contains("Elena"));
    }
}
