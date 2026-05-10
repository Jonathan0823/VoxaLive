//! Provider factory for backend routes.

use voxalive_config::{models::LlmProviderKind, models::TtsProviderKind, ConfigManager};
use voxalive_core::domain::CoreError;
use voxalive_providers::frontend::vts::VtsAdapter;
use voxalive_providers::llm::adapters::{GeminiAdapter, OllamaAdapter, OllamaMode, OpenRouterAdapter};
use voxalive_providers::llm::LlmProvider;
use voxalive_providers::stt::adapters::WhisperAdapter;
use voxalive_providers::stt::SttProvider;
use voxalive_providers::tts::adapters::PiperAdapter;
use voxalive_providers::tts::TtsProvider;

#[derive(Debug, Default, Clone)]
pub struct ProviderFactory;

impl ProviderFactory {
    pub async fn llm_provider(&self, config: &ConfigManager) -> Result<Box<dyn LlmProvider>, CoreError> {
        let runtime = config.runtime_config();
        let provider: Box<dyn LlmProvider> = match runtime.llm.provider {
            LlmProviderKind::Gemini => {
                let api_key = config
                    .secret_value("gemini_api_key")
                    .ok_or_else(|| CoreError::new("PROVIDER_NOT_CONFIGURED", "Gemini API key missing"))?;
                Box::new(GeminiAdapter::new(api_key, runtime.llm.model.clone()))
            }
            LlmProviderKind::OpenRouter => {
                let api_key = config
                    .secret_value("openrouter_api_key")
                    .ok_or_else(|| CoreError::new("PROVIDER_NOT_CONFIGURED", "OpenRouter API key missing"))?;
                Box::new(OpenRouterAdapter::new(api_key, runtime.llm.model.clone()))
            }
            LlmProviderKind::Ollama => Box::new(OllamaAdapter::new(
                "http://127.0.0.1:11434",
                runtime.llm.model.clone(),
                OllamaMode::Native,
            )),
        };

        Ok(provider)
    }

    pub fn tts_provider(&self, config: &ConfigManager) -> Result<Box<dyn TtsProvider>, CoreError> {
        let runtime = config.runtime_config();
        let provider: Box<dyn TtsProvider> = match runtime.tts.provider {
            TtsProviderKind::Piper => {
                let model_path = runtime
                    .tts
                    .model_path
                    .clone()
                    .ok_or_else(|| CoreError::new("PROVIDER_NOT_CONFIGURED", "Piper model path missing"))?;
                Box::new(PiperAdapter::new("piper", model_path))
            }
            TtsProviderKind::Qwen => {
                return Err(CoreError::new(
                    "PROVIDER_NOT_CONFIGURED",
                    "Qwen TTS adapter is not implemented in this MVP",
                ));
            }
        };

        Ok(provider)
    }

    pub fn stt_provider(&self, config: &ConfigManager) -> Result<Box<dyn SttProvider>, CoreError> {
        let runtime = config.runtime_config();
        let model_path = runtime
            .stt
            .model_path
            .clone()
            .ok_or_else(|| CoreError::new("PROVIDER_NOT_CONFIGURED", "Whisper model path missing"))?;
        let provider = WhisperAdapter::new(model_path)?;
        Ok(Box::new(provider))
    }

    pub fn vts_adapter(&self, config: &ConfigManager) -> VtsAdapter {
        let runtime = config.runtime_config();
        let auth_token = config.secret_value("vts_auth_token");

        VtsAdapter::new(
            runtime.server.vts_endpoint.clone(),
            runtime.server.vts_plugin_name.clone(),
            runtime.server.vts_plugin_developer.clone(),
            auth_token,
        )
    }
}
