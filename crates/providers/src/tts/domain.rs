//! TTS domain types.

use serde::{Deserialize, Serialize};

/// TTS request payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub provider: Option<String>,
}

/// TTS response payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TtsResponse {
    pub audio_format: String,
    pub audio_bytes: Vec<u8>,
}
