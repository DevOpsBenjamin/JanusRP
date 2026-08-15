use async_trait::async_trait;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

use crate::client::{LlmClient, NarrationStream};
use crate::error::LlmError;
use crate::types::{DirectorBriefing, MjArbitrationResponse, ToolCall, TurnPrompt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpLlmConfig {
    pub glimmer_base_url: String,
    pub glimmer_model: String,
    pub glimmer_api_key: Option<String>,
    pub qwen_base_url: String,
    pub qwen_model: String,
    pub qwen_api_key: Option<String>,
    pub timeout_seconds: u64,
    pub temperature_arbitration: Option<f32>,
    pub temperature_narration: Option<f32>,
}

impl Default for HttpLlmConfig {
    fn default() -> Self {
        Self {
            glimmer_base_url: "http://localhost:8000/v1".to_string(),
            glimmer_model: "meta-muse-glimmer-30b".to_string(),
            glimmer_api_key: None,
            qwen_base_url: "http://localhost:8001/v1".to_string(),
            qwen_model: "qwen-3.8".to_string(),
            qwen_api_key: None,
            timeout_seconds: 60,
            temperature_arbitration: Some(0.2),
            temperature_narration: Some(0.7),
        }
    }
}

impl HttpLlmConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(val) = std::env::var("GLIMMER_BASE_URL").or_else(|_| std::env::var("LLM_BASE_URL")) {
            config.glimmer_base_url = val;
        }
        if let Ok(val) = std::env::var("GLIMMER_MODEL").or_else(|_| std::env::var("LLM_MODEL")) {
            config.glimmer_model = val;
        }
        if let Ok(val) = std::env::var("GLIMMER_API_KEY").or_else(|_| std::env::var("LLM_API_KEY")) {
            config.glimmer_api_key = Some(val);
        }

        if let Ok(val) = std::env::var("QWEN_BASE_URL").or_else(|_| std::env::var("LLM_BASE_URL")) {
            config.qwen_base_url = val;
        }
        if let Ok(val) = std::env::var("QWEN_MODEL").or_else(|_| std::env::var("LLM_MODEL")) {
            config.qwen_model = val;
        }
        if let Ok(val) = std::env::var("QWEN_API_KEY").or_else(|_| std::env::var("LLM_API_KEY")) {
            config.qwen_api_key = Some(val);
        }

        if let Ok(val) = std::env::var("LLM_TIMEOUT_SECONDS") {
            if let Ok(parsed) = val.parse::<u64>() {
                config.timeout_seconds = parsed;
            }
        }

        config
    }

    pub fn with_glimmer(mut self, base_url: impl Into<String>, model: impl Into<String>) -> Self {
        self.glimmer_base_url = base_url.into();
        self.glimmer_model = model.into();
        self
    }

    pub fn with_qwen(mut self, base_url: impl Into<String>, model: impl Into<String>) -> Self {
        self.qwen_base_url = base_url.into();
        self.qwen_model = model.into();
        self
    }

    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
}

#[derive(Debug, Clone)]
pub struct HttpLlmClient {
    client: reqwest::Client,
    config: HttpLlmConfig,
}

impl HttpLlmClient {
    pub fn new(config: HttpLlmConfig) -> Result<Self, LlmError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()?;
        Ok(Self { client, config })
    }

    pub fn from_env() -> Result<Self, LlmError> {
        Self::new(HttpLlmConfig::from_env())
    }

    pub fn config(&self) -> &HttpLlmConfig {
        &self.config
    }
}

#[async_trait]
impl LlmClient for HttpLlmClient {
    async fn complete_turn_arbitration(
        &self,
        prompt: &TurnPrompt,
    ) -> Result<MjArbitrationResponse, LlmError> {
        let url = format!(
            "{}/chat/completions",
            self.config.glimmer_base_url.trim_end_matches('/')
        );

        let tools_schema = janus_mcp::tools::get_turn_tools_schema();
        let tools_json: Vec<serde_json::Value> = tools_schema
            .into_iter()
            .map(|t| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            })
            .collect();

        let user_content = format!(
            "Context:\n{}\n\nPlayer Input:\n{}",
            prompt.context_summary, prompt.player_input
        );

        let mut payload = serde_json::json!({
            "model": self.config.glimmer_model,
            "messages": [
                {
                    "role": "system",
                    "content": prompt.system_prompt,
                },
                {
                    "role": "user",
                    "content": user_content,
                }
            ],
            "tools": tools_json,
            "tool_choice": "auto",
        });

        if let Some(temp) = self.config.temperature_arbitration {
            payload["temperature"] = serde_json::json!(temp);
        }

        let mut req = self.client.post(&url).json(&payload);
        if let Some(ref key) = self.config.glimmer_api_key {
            req = req.bearer_auth(key);
        }

        let res = req.send().await.map_err(LlmError::Http)?;
        let status = res.status();
        if !status.is_success() {
            let err_body = res.text().await.unwrap_or_default();
            return Err(LlmError::Api {
                status: status.as_u16(),
                message: err_body,
            });
        }

        let body: serde_json::Value = res.json().await.map_err(LlmError::Http)?;

        let choice = body
            .get("choices")
            .and_then(|c| c.get(0))
            .ok_or_else(|| LlmError::InvalidResponse("Missing 'choices[0]' in API response".to_string()))?;

        let message = choice
            .get("message")
            .ok_or_else(|| LlmError::InvalidResponse("Missing 'message' in choice".to_string()))?;

        let raw_content = message
            .get("content")
            .and_then(|c| c.as_str())
            .unwrap_or_default();

        let mut tool_calls = Vec::new();
        if let Some(calls) = message.get("tool_calls").and_then(|tc| tc.as_array()) {
            for call in calls {
                if let Some(func) = call.get("function") {
                    let name = func
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or_default()
                        .to_string();

                    let arguments = match func.get("arguments") {
                        Some(serde_json::Value::String(s)) => {
                            serde_json::from_str::<serde_json::Value>(s).unwrap_or_else(|_| {
                                serde_json::json!({ "raw": s })
                            })
                        }
                        Some(v) => v.clone(),
                        None => serde_json::Value::Null,
                    };

                    tool_calls.push(ToolCall { name, arguments });
                }
            }
        }

        let (reasoning, director_briefing) = extract_reasoning_and_briefing(raw_content);

        Ok(MjArbitrationResponse {
            reasoning,
            tool_calls,
            director_briefing,
        })
    }

    async fn stream_narration(
        &self,
        briefing: &DirectorBriefing,
    ) -> Result<NarrationStream, LlmError> {
        let url = format!(
            "{}/chat/completions",
            self.config.qwen_base_url.trim_end_matches('/')
        );

        let user_content = format!(
            "Consigne du Directeur:\n{}\n\nContexte:\n{}",
            briefing.briefing_instructions,
            serde_json::to_string_pretty(&briefing.context).unwrap_or_default()
        );

        let mut payload = serde_json::json!({
            "model": self.config.qwen_model,
            "messages": [
                {
                    "role": "system",
                    "content": briefing.system_prompt,
                },
                {
                    "role": "user",
                    "content": user_content,
                }
            ],
            "stream": true,
        });

        if let Some(temp) = self.config.temperature_narration {
            payload["temperature"] = serde_json::json!(temp);
        }

        let mut req = self.client.post(&url).json(&payload);
        if let Some(ref key) = self.config.qwen_api_key {
            req = req.bearer_auth(key);
        }

        let res = req.send().await.map_err(LlmError::Http)?;
        let status = res.status();
        if !status.is_success() {
            let err_body = res.text().await.unwrap_or_default();
            return Err(LlmError::Api {
                status: status.as_u16(),
                message: err_body,
            });
        }

        let (tx, rx) = tokio::sync::mpsc::channel(64);
        let mut byte_stream = res.bytes_stream();

        tokio::spawn(async move {
            let mut buffer: Vec<u8> = Vec::new();

            while let Some(item) = byte_stream.next().await {
                match item {
                    Ok(bytes) => {
                        buffer.extend_from_slice(&bytes);
                        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                            let line_bytes = buffer[..pos].to_vec();
                            buffer.drain(..=pos);
                            let line = String::from_utf8_lossy(&line_bytes).trim().to_string();
                            if line.is_empty() {
                                continue;
                            }
                            if line.starts_with("data:") {
                                let data = line.trim_start_matches("data:").trim();
                                if data == "[DONE]" {
                                    return;
                                }
                                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(data) {
                                    if let Some(content) = json_val
                                        .get("choices")
                                        .and_then(|c| c.get(0))
                                        .and_then(|c| c.get("delta"))
                                        .and_then(|d| d.get("content"))
                                        .and_then(|s| s.as_str())
                                    {
                                        if !content.is_empty()
                                            && tx.send(Ok(content.to_string())).await.is_err()
                                        {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(LlmError::Http(e))).await;
                        return;
                    }
                }
            }

            // Process any remaining bytes without newline
            if !buffer.is_empty() {
                let line = String::from_utf8_lossy(&buffer).trim().to_string();
                if line.starts_with("data:") {
                    let data = line.trim_start_matches("data:").trim();
                    if data != "[DONE]" {
                        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(content) = json_val
                                .get("choices")
                                .and_then(|c| c.get(0))
                                .and_then(|c| c.get("delta"))
                                .and_then(|d| d.get("content"))
                                .and_then(|s| s.as_str())
                            {
                                if !content.is_empty() {
                                    let _ = tx.send(Ok(content.to_string())).await;
                                }
                            }
                        }
                    }
                }
            }
        });

        let receiver_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Box::pin(receiver_stream))
    }
}

pub fn extract_reasoning_and_briefing(content: &str) -> (String, String) {
    let trimmed = content.trim();

    // 1. JSON parsing check
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(trimmed) {
            let reasoning = json_val
                .get("reasoning")
                .and_then(|r| r.as_str())
                .unwrap_or_default()
                .to_string();

            let briefing = json_val
                .get("director_briefing")
                .or_else(|| json_val.get("briefing"))
                .and_then(|b| b.as_str())
                .unwrap_or(trimmed)
                .to_string();

            if !reasoning.is_empty() || !briefing.is_empty() {
                return (reasoning, briefing);
            }
        }
    }

    // 2. XML tags extraction (<thought> / <reasoning> and <briefing>)
    let has_thought = trimmed.contains("<thought>") || trimmed.contains("<reasoning>");
    let has_briefing = trimmed.contains("<briefing>");

    if has_thought || has_briefing {
        let reasoning = if let Some(start) = trimmed.find("<thought>").or_else(|| trimmed.find("<reasoning>")) {
            let tag_len = if trimmed[start..].starts_with("<thought>") { "<thought>".len() } else { "<reasoning>".len() };
            let end_tag = if trimmed[start..].starts_with("<thought>") { "</thought>" } else { "</reasoning>" };
            if let Some(end) = trimmed[start + tag_len..].find(end_tag) {
                trimmed[start + tag_len..start + tag_len + end].trim().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let briefing = if let Some(start) = trimmed.find("<briefing>") {
            let tag_len = "<briefing>".len();
            if let Some(end) = trimmed[start + tag_len..].find("</briefing>") {
                trimmed[start + tag_len..start + tag_len + end].trim().to_string()
            } else {
                trimmed[start + tag_len..].trim().to_string()
            }
        } else {
            // Remove reasoning tags if any
            let cleaned = if let Some(start) = trimmed.find("<thought>").or_else(|| trimmed.find("<reasoning>")) {
                let end_tag = if trimmed[start..].starts_with("<thought>") { "</thought>" } else { "</reasoning>" };
                if let Some(end) = trimmed[start..].find(end_tag) {
                    let after = &trimmed[start + end + end_tag.len()..];
                    let before = &trimmed[..start];
                    format!("{} {}", before.trim(), after.trim()).trim().to_string()
                } else {
                    trimmed.to_string()
                }
            } else {
                trimmed.to_string()
            };
            cleaned
        };

        return (reasoning, briefing);
    }

    // 3. Fallback: plain text
    (trimmed.to_string(), trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_reasoning_and_briefing_xml() {
        let raw = "<thought>Le joueur veut séduire Elena.</thought>\n<briefing>Elena rougit légèrement.</briefing>";
        let (reasoning, briefing) = extract_reasoning_and_briefing(raw);
        assert_eq!(reasoning, "Le joueur veut séduire Elena.");
        assert_eq!(briefing, "Elena rougit légèrement.");
    }

    #[test]
    fn test_extract_reasoning_and_briefing_json() {
        let raw = r#"{"reasoning": "Arbitrage réussi", "director_briefing": "Décris l'auberge"}"#;
        let (reasoning, briefing) = extract_reasoning_and_briefing(raw);
        assert_eq!(reasoning, "Arbitrage réussi");
        assert_eq!(briefing, "Décris l'auberge");
    }

    #[test]
    fn test_extract_reasoning_and_briefing_plain() {
        let raw = "Elena observe le joueur d'un air curieux.";
        let (reasoning, briefing) = extract_reasoning_and_briefing(raw);
        assert_eq!(reasoning, raw);
        assert_eq!(briefing, raw);
    }

    #[test]
    fn test_http_config_builder() {
        let cfg = HttpLlmConfig::default()
            .with_glimmer("http://my-vllm:8000/v1", "glimmer-custom")
            .with_qwen("http://my-aphrodite:8001/v1", "qwen-custom")
            .with_timeout(30);

        assert_eq!(cfg.glimmer_base_url, "http://my-vllm:8000/v1");
        assert_eq!(cfg.glimmer_model, "glimmer-custom");
        assert_eq!(cfg.qwen_base_url, "http://my-aphrodite:8001/v1");
        assert_eq!(cfg.qwen_model, "qwen-custom");
        assert_eq!(cfg.timeout_seconds, 30);
    }
}
