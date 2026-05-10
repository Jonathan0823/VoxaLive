use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
use voxalive_core::domain::CoreError;

#[derive(Debug, Clone)]
pub struct OpenRouterAdapter {
    api_key: String,
    model: String,
    referer: Option<String>,
    title: Option<String>,
    client: Client,
}

impl OpenRouterAdapter {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            referer: None,
            title: None,
            client: Client::new(),
        }
    }

    pub fn with_attribution(mut self, referer: Option<String>, title: Option<String>) -> Self {
        self.referer = referer;
        self.title = title;
        self
    }

    fn endpoint(&self) -> &'static str {
        "https://openrouter.ai/api/v1/chat/completions"
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("OPENROUTER_PROVIDER_ERROR", message)
    }
}

#[async_trait]
impl LlmProvider for OpenRouterAdapter {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, CoreError> {
        let payload = OpenRouterRequest::from_prompt(&self.model, &request.prompt);
        let mut builder = self
            .client
            .post(self.endpoint())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload);

        if let Some(referer) = &self.referer {
            builder = builder.header("HTTP-Referer", referer);
        }

        if let Some(title) = &self.title {
            builder = builder.header("X-OpenRouter-Title", title);
        }

        let response = builder
            .send()
            .await
            .and_then(|resp| resp.error_for_status())
            .map_err(|err| Self::map_error(err.to_string()))?;

        let body: OpenRouterResponse = response
            .json()
            .await
            .map_err(|err| Self::map_error(err.to_string()))?;

        let text = body
            .choices
            .into_iter()
            .find_map(|choice| choice.message.content)
            .ok_or_else(|| Self::map_error("OpenRouter response did not contain text"))?;

        Ok(LlmResponse { text })
    }
}

#[derive(Debug, Serialize)]
struct OpenRouterRequest<'a> {
    model: &'a str,
    messages: Vec<OpenRouterMessage<'a>>,
}

impl<'a> OpenRouterRequest<'a> {
    fn from_prompt(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            messages: vec![OpenRouterMessage { role: "user", content: prompt }],
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenRouterMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<OpenRouterChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoiceMessage {
    content: Option<String>,
}
