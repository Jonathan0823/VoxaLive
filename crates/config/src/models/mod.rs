//! Config models and validation.
//!
//! Runtime config, LLM config, TTS config, etc.
//! See docs/09_CONFIG_AND_SECRETS.md for the full spec.

use serde::{Deserialize, Serialize};

/// Runtime config that can be updated via API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub llm: LlmConfig,
    pub tts: TtsConfig,
    pub stt: SttConfig,
    pub live: LiveConfig,
    pub server: ServerConfig,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            llm: LlmConfig::default(),
            tts: TtsConfig::default(),
            stt: SttConfig::default(),
            live: LiveConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

/// LLM provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "gemini".to_string(),
            model: "gemini-2.0-flash-exp".to_string(),
            temperature: 0.7,
            max_tokens: 1024,
        }
    }
}

/// TTS provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub mode: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_path: Option<String>,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            mode: "cpu".to_string(),
            provider: "piper".to_string(),
            model_path: Some("./voices/default.onnx".to_string()),
        }
    }
}

/// STT configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    pub device: String,
}

impl Default for SttConfig {
    fn default() -> Self {
        Self {
            device: "cpu".to_string(),
        }
    }
}

/// Live input configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveConfig {
    pub enabled: bool,
    pub youtube_video_id: String,
    pub tiktok_room: String,
}

impl Default for LiveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            youtube_video_id: String::new(),
            tiktok_room: String::new(),
        }
    }
}

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub admin_ui_enabled: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            admin_ui_enabled: true,
        }
    }
}

/// Valid LLM providers.
pub const VALID_LLM_PROVIDERS: &[&str] = &["gemini", "openrouter", "ollama"];

/// Valid TTS modes.
pub const VALID_TTS_MODES: &[&str] = &["cpu", "gpu", "auto"];

/// Valid STT devices.
pub const VALID_STT_DEVICES: &[&str] = &["cpu", "cuda:0", "auto"];

/// Validate runtime config.
pub fn validate_config(config: &RuntimeConfig) -> Result<(), String> {
    // Validate LLM
    if !VALID_LLM_PROVIDERS.contains(&config.llm.provider.as_str()) {
        return Err(format!("Invalid LLM provider: {}", config.llm.provider));
    }
    if config.llm.temperature < 0.0 || config.llm.temperature > 2.0 {
        return Err("LLM temperature must be between 0.0 and 2.0".to_string());
    }
    if config.llm.max_tokens == 0 {
        return Err("LLM max_tokens must be greater than 0".to_string());
    }
    if config.llm.model.is_empty() {
        return Err("LLM model name must not be empty".to_string());
    }

    // Validate TTS
    if !VALID_TTS_MODES.contains(&config.tts.mode.as_str()) {
        return Err(format!("Invalid TTS mode: {}", config.tts.mode));
    }

    // Validate STT
    if !VALID_STT_DEVICES.contains(&config.stt.device.as_str()) {
        return Err(format!("Invalid STT device: {}", config.stt.device));
    }

    Ok(())
}
