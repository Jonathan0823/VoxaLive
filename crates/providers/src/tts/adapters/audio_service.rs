use std::sync::OnceLock;

use crate::tts::{TtsProvider, TtsRequest, TtsResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;
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

        let service_url = self.service_url.clone();

        tokio::task::block_in_place(|| {
            Handle::current().block_on(async move {
                let response = http_client()
                    .post(&format!("{}/tts/synthesize", service_url))
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| {
                        CoreError::new(
                            "TTS_SERVICE_ERROR",
                            format!("Failed to call TTS service: {}", e),
                        )
                    })?;

                if !response.status().is_success() {
                    return Err(CoreError::new(
                        "TTS_SERVICE_ERROR",
                        format!("TTS service returned error: {}", response.status()),
                    ));
                }

                let result: SynthesizeResponse = response
                    .json()
                    .await
                    .map_err(|e| {
                        CoreError::new(
                            "TTS_SERVICE_ERROR",
                            format!("Failed to parse TTS response: {}", e),
                        )
                    })?;

                use base64::Engine;
                let audio_bytes = base64::engine::general_purpose::STANDARD
                    .decode(&result.audio_data)
                    .map_err(|e| {
                        CoreError::new(
                            "TTS_SERVICE_ERROR",
                            format!("Failed to decode audio: {}", e),
                        )
                    })?;

                Ok(TtsResponse {
                    audio_format: result.format,
                    audio_bytes,
                })
            })
        })
    }
}
