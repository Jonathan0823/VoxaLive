use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
use voxalive_core::domain::CoreError;

#[derive(Debug, Clone)]
pub struct GeminiAdapter {
    api_key: String,
    model: String,
    client: Client,
}

impl GeminiAdapter {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            client: Client::new(),
        }
    }

    fn endpoint(&self) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        )
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("GEMINI_PROVIDER_ERROR", message)
    }
}

impl LlmProvider for GeminiAdapter {
    fn generate(&self, request: LlmRequest) -> Result<LlmResponse, CoreError> {
        let payload = GeminiRequest::from_prompt(&request.prompt);
        let response = self
            .client
            .post(self.endpoint())
            .header("x-goog-api-key", &self.api_key)
            .json(&payload)
            .send()
            .and_then(|resp| resp.error_for_status())
            .map_err(|err| Self::map_error(err.to_string()))?;

        let body: GeminiResponse = response
            .json()
            .map_err(|err| Self::map_error(err.to_string()))?;

        let text = body
            .candidates
            .into_iter()
            .flat_map(|candidate| candidate.content.parts)
            .find_map(|part| part.text)
            .ok_or_else(|| Self::map_error("Gemini response did not contain text"))?;

        Ok(LlmResponse { text })
    }
}

#[derive(Debug, Serialize)]
struct GeminiRequest<'a> {
    contents: Vec<GeminiContent<'a>>,
}

impl<'a> GeminiRequest<'a> {
    fn from_prompt(prompt: &'a str) -> Self {
        Self {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart { text: prompt }],
            }],
        }
    }
}

#[derive(Debug, Serialize)]
struct GeminiContent<'a> {
    parts: Vec<GeminiPart<'a>>,
}

#[derive(Debug, Serialize)]
struct GeminiPart<'a> {
    text: &'a str,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiCandidateContent,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidateContent {
    parts: Vec<GeminiCandidatePart>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidatePart {
    text: Option<String>,
}
