//! LLM domain types.

use serde::{Deserialize, Serialize};

/// LLM request payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmRequest {
    pub prompt: String,
    pub provider: Option<String>,
}

/// LLM response payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
}
