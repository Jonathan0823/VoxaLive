//! LLM domain types.



/// LLM request payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmRequest {
    pub prompt: String,
    pub provider: Option<String>,
}

/// LLM response payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmResponse {
    pub text: String,
}