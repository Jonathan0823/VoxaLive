//! STT domain types.

use serde::{Deserialize, Serialize};

/// STT request payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SttRequest {
    pub audio_format: String,
    pub sample_rate: u32,
    pub channels: u32,
    pub audio_bytes: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// STT response payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SttResponse {
    pub transcript: String,
}
