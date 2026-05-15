//! API request/response DTOs.
//!
//! All public API types should live in this module.
//! See docs/07_API_CONTRACT.md for the full contract.

use serde::{Deserialize, Serialize};

/// Standard API success response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    /// Create a success response.
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create an error response.
    pub fn error(code: &str, message: &str) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(ApiError {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }
}

/// API error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

/// Error codes from docs/07_API_CONTRACT.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    Unauthorized,
    Forbidden,
    ConfigValidationFailed,
    SecretUpdateFailed,
    ProviderNotConfigured,
    ProviderTestFailed,
    WsProtocolError,
    InternalError,
}

// ========== Health ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_sec: u64,
    pub active_llm_provider: String,
    pub active_tts_provider: String,
}

// ========== Config ==========

/// LLM config for API contract (provider as string).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

/// TTS config for API contract (provider as string).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub mode: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_path: Option<String>,
}

/// STT config for API contract (device as string).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_path: Option<String>,
}

/// Live input config for API contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveConfig {
    pub enabled: bool,
    pub youtube_video_id: String,
    pub tiktok_room: String,
}

/// Server config for API contract (full + patch).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub vts_endpoint: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigData {
    pub llm: LlmConfig,
    pub tts: TtsConfig,
    pub stt: SttConfig,
    pub live: LiveConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub data: ConfigData,
}

// ========== Config Patch ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPatchConfig {
    pub vts_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vts_plugin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vts_plugin_developer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<LlmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stt: Option<SttConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<LiveConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerPatchConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPatchResponse {
    pub updated: bool,
}

// ========== Secrets ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStatusResponse {
    pub secrets: std::collections::HashMap<String, SecretStatus>,
}

/// Secret status for a single key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStatus {
    pub configured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemini_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openrouter_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vts_auth_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretUpdateResponse {
    pub updated: Vec<String>,
}

// ========== Provider Test ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmTestRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmTestResponse {
    pub provider: String,
    pub model: String,
    pub latency_ms: u64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsTestRequest {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsTestResponse {
    pub provider: String,
    pub latency_ms: u64,
    pub audio_format: String,
    /// Base64-encoded audio data for playback in the test console.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_base64: Option<String>,
}

/// Generic test response for simple provider tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResponse {
    pub provider: String,
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}
