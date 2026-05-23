use std::sync::OnceLock;

use crate::tts::{TtsProvider, TtsRequest, TtsResponse};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use voxalive_core::domain::CoreError;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| Client::new())
}

#[derive(Debug, Clone)]
pub struct AudioServiceTtsAdapter {
    service_url: String,
}

#[derive(Serialize)]
struct SynthesizeRequest {
    text: String,
    voice_id: String,
    format: String,
}

#[derive(Deserialize)]
struct SynthesizeResponse {
    audio_data: String,
    sample_rate: i32,
    channels: i32,
    format: String,
}

impl AudioServiceTtsAdapter {
    pub fn new(service_url: impl Into<String>) -> Self {
        Self {
            service_url: service_url.into(),
        }
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("TTS_SERVICE_ERROR", message)
    }
}

impl TtsProvider for AudioServiceTtsAdapter {
    fn synthesize(&self, request: TtsRequest) -> Result<TtsResponse, CoreError> {
        let req = SynthesizeRequest {
            text: request.text,
            voice_id: "default".to_string(),
            format: "wav".to_string(),
        };

        let response = http_client()
            .post(&format!("{}/tts/synthesize", self.service_url))
            .json(&req)
            .send()
            .map_err(|e| Self::map_error(format!("Failed to call TTS service: {}", e)))?;

        if !response.status().is_success() {
            return Err(Self::map_error(format!(
                "TTS service returned error: {}",
                response.status()
            )));
        }

        let result: SynthesizeResponse = response
            .json()
            .map_err(|e| Self::map_error(format!("Failed to parse TTS response: {}", e)))?;

        use base64::Engine;
        let audio_bytes = base64::engine::general_purpose::STANDARD
            .decode(&result.audio_data)
            .map_err(|e| Self::map_error(format!("Failed to decode audio: {}", e)))?;

        Ok(TtsResponse {
            audio_format: result.format,
            audio_bytes,
        })
    }
}
