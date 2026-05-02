//! STT domain types.



/// STT request payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SttRequest {
    pub audio_format: String,
    pub sample_rate: u32,
    pub channels: u32,
}

/// STT response payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SttResponse {
    pub transcript: String,
}