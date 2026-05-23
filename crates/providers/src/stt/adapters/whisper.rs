//! Whisper STT adapter that calls external audio-inference service.
//!
//! This adapter communicates with the audio-inference service via HTTP.
//! The service handles Whisper model loading and inference, keeping CUDA
//! complexity out of the Rust workspace.

use std::sync::OnceLock;

use crate::stt::{SttProvider, SttRequest, SttResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

use voxalive_core::domain::CoreError;

/// Global HTTP client for STT service calls.
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| Client::new())
}

/// Whisper adapter that calls external audio-inference service.
#[derive(Debug)]
pub struct WhisperAdapter {
    service_url: String,
}

#[derive(Serialize)]
struct TranscribeRequest {
    audio_format: String,
    sample_rate: u32,
    channels: u32,
    language: Option<String>,
    audio_data: String, // base64 encoded
}

#[derive(Deserialize)]
struct TranscribeResponse {
    transcript: String,
}

impl WhisperAdapter {
    pub fn new(service_url: impl Into<String>) -> Self {
        Self {
            service_url: service_url.into(),
        }
    }

    fn map_error(message: impl Into<String>) -> CoreError {
        CoreError::new("STT_SERVICE_ERROR", message)
    }
}

impl SttProvider for WhisperAdapter {
    fn transcribe(&self, request: SttRequest) -> Result<SttResponse, CoreError> {
        let audio_data = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &request.audio_bytes,
        );

        let req = TranscribeRequest {
            audio_format: request.audio_format,
            sample_rate: request.sample_rate,
            channels: request.channels,
            language: request.language,
            audio_data,
        };

        let service_url = self.service_url.clone();

        tokio::task::block_in_place(|| {
            Handle::current().block_on(async move {
                let response = http_client()
                    .post(&format!("{}/stt/transcribe", service_url))
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| {
                        CoreError::new(
                            "STT_SERVICE_ERROR",
                            format!("Failed to call STT service: {}", e),
                        )
                    })?;

                if !response.status().is_success() {
                    return Err(CoreError::new(
                        "STT_SERVICE_ERROR",
                        format!("STT service returned error: {}", response.status()),
                    ));
                }

                let result: TranscribeResponse = response.json().await.map_err(|e| {
                    CoreError::new(
                        "STT_SERVICE_ERROR",
                        format!("Failed to parse STT response: {}", e),
                    )
                })?;

                Ok(SttResponse {
                    transcript: result.transcript,
                })
            })
        })
    }
}

/// Async version of WhisperAdapter for use in async contexts.
pub struct AsyncWhisperAdapter {
    service_url: String,
    client: reqwest::Client,
}

impl AsyncWhisperAdapter {
    pub fn new(service_url: impl Into<String>) -> Self {
        Self {
            service_url: service_url.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn transcribe(&self, request: SttRequest) -> Result<SttResponse, CoreError> {
        // Encode audio as base64
        let audio_data = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &request.audio_bytes,
        );

        let req = TranscribeRequest {
            audio_format: request.audio_format,
            sample_rate: request.sample_rate,
            channels: request.channels,
            language: request.language,
            audio_data,
        };

        let response = self
            .client
            .post(&format!("{}/stt/transcribe", self.service_url))
            .json(&req)
            .send()
            .await
            .map_err(|e| CoreError::new("STT_SERVICE_ERROR", format!("Failed to call STT service: {}", e)))?;

        if !response.status().is_success() {
            return Err(CoreError::new(
                "STT_SERVICE_ERROR",
                format!("STT service returned error: {}", response.status()),
            ));
        }

        let result: TranscribeResponse = response
            .json()
            .await
            .map_err(|e| CoreError::new("STT_SERVICE_ERROR", format!("Failed to parse STT response: {}", e)))?;

        Ok(SttResponse { transcript: result.transcript })
    }
}