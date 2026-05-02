//! Core shared domain types.
//!
//! Only types shared across ALL adapters go here.
//! Adapter-specific domain types live in crates/providers.

/// Input kind accepted by the pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputKind {
    Text,
    Audio,
    LiveComment,
}

/// Normalized input for the pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPayload {
    pub kind: InputKind,
    pub request_id: String,
    pub text: Option<String>,
    pub audio_format: Option<String>,
}

/// Core error type - shared across all adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreError {
    pub code: String,
    pub message: String,
}

impl CoreError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for CoreError {}