use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
use voxalive_core::domain::CoreError;

#[derive(Debug, Clone, Copy)]
pub enum OllamaMode {
    Native,
    OpenAICompatible,
}

#[derive(Debug, Clone)]
pub struct OllamaAdapter {
    base_url: String,
    model: String,
    mode: OllamaMode,
    client: Client,
}

impl OllamaAdapter {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>, mode: OllamaMode) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            mode,
            client: Client::new(),
        }
    }

    fn endpoint(&self) -> String {
        match self.mode {
            OllamaMode::Native => format!("{}/api/chat", self.base_url.trim_end_matches('/')),
            OllamaMode::OpenAICompatible => {
                format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'))
            }
        }
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("OLLAMA_PROVIDER_ERROR", message)
    }
}

#[async_trait]
impl LlmProvider for OllamaAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, CoreError> {
        let response = match self.mode {
            OllamaMode::Native => {
                let payload = OllamaNativeRequest::from_prompt(&self.model, &request.prompt);
                let resp = self
                    .client
                    .post(self.endpoint())
                    .json(&payload)
                    .send()
                    .await
                    .and_then(|r| r.error_for_status())
                    .map_err(|err| Self::map_error(err.to_string()))?;
                resp.json::<OllamaNativeResponse>()
                    .await
                    .map_err(|err| Self::map_error(err.to_string()))?
                    .message
                    .content
            }
            OllamaMode::OpenAICompatible => {
                let payload = OllamaOpenAIRequest::from_prompt(&self.model, &request.prompt);
                let resp = self
                    .client
                    .post(self.endpoint())
                    .json(&payload)
                    .send()
                    .await
                    .and_then(|r| r.error_for_status())
                    .map_err(|err| Self::map_error(err.to_string()))?;
                resp.json::<OllamaOpenAIResponse>()
                    .await
                    .map_err(|err| Self::map_error(err.to_string()))?
                    .choices
                    .into_iter()
                    .find_map(|choice| choice.message.content)
                    .ok_or_else(|| Self::map_error("Ollama response did not contain text"))?
            }
        };

        Ok(LlmResponse { text: response })
    }
}

#[derive(Debug, Serialize)]
struct OllamaNativeRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
}

impl<'a> OllamaNativeRequest<'a> {
    fn from_prompt(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            messages: vec![OllamaMessage { role: "user", content: prompt }],
        }
    }
}

#[derive(Debug, Serialize)]
struct OllamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct OllamaNativeResponse {
    message: OllamaNativeMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaNativeMessage {
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
}

impl<'a> OllamaOpenAIRequest<'a> {
    fn from_prompt(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            messages: vec![OllamaMessage { role: "user", content: prompt }],
        }
    }
}

#[derive(Debug, Deserialize)]
struct OllamaOpenAIResponse {
    choices: Vec<OllamaOpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OllamaOpenAIChoice {
    message: OllamaOpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaOpenAIMessage {
    content: Option<String>,
}
