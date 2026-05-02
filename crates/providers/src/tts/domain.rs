//! TTS domain types.



/// TTS request payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TtsRequest {
    pub text: String,
    pub provider: Option<String>,
}

/// TTS response payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TtsResponse {
    pub audio_format: String,
    pub audio_bytes: Vec<u8>,
}