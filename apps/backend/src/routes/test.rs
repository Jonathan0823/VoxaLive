//! Provider test endpoints.

use axum::{extract::State, http::StatusCode, Json};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use voxalive_protocol::api::{ApiResponse, LlmTestRequest, LlmTestResponse, TestResponse, TtsTestRequest, TtsTestResponse};

use crate::state::AppState;
use voxalive_providers::frontend::FrontendAdapter;
use voxalive_providers::llm::LlmRequest;
use voxalive_providers::stt::SttRequest;
use voxalive_providers::tts::TtsRequest;

/// POST /api/test/llm - Test the active LLM provider.
pub async fn test_llm(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LlmTestRequest>,
) -> (StatusCode, Json<ApiResponse<LlmTestResponse>>) {
    let state = state.lock().await;
    let provider = match state.providers.llm_provider(&state.config).await {
        Ok(provider) => provider,
        Err(err) => return (StatusCode::BAD_REQUEST, Json(ApiResponse::<LlmTestResponse>::error("PROVIDER_NOT_CONFIGURED", &err.to_string()))),
    };

    let start = Instant::now();
    match provider.generate(LlmRequest { prompt: payload.message, provider: payload.provider }).await {
        Ok(response) => (
            StatusCode::OK,
            Json(ApiResponse::ok(LlmTestResponse {
                provider: format!("{:?}", state.config.runtime_config().llm.provider),
                model: state.config.runtime_config().llm.model.clone(),
                latency_ms: start.elapsed().as_millis() as u64,
                text: response.text,
            })),
        ),
        Err(err) => (StatusCode::BAD_GATEWAY, Json(ApiResponse::<LlmTestResponse>::error("PROVIDER_TEST_FAILED", &err.to_string()))),
    }
}

/// POST /api/test/tts - Test the active TTS provider.
pub async fn test_tts(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<TtsTestRequest>,
) -> (StatusCode, Json<ApiResponse<TtsTestResponse>>) {
    let state = state.lock().await;
    let provider = match state.providers.tts_provider(&state.config) {
        Ok(provider) => provider,
        Err(err) => return (StatusCode::BAD_REQUEST, Json(ApiResponse::<TtsTestResponse>::error("PROVIDER_NOT_CONFIGURED", &err.to_string()))),
    };

    let start = Instant::now();
    match provider.synthesize(TtsRequest { text: payload.text, provider: payload.provider }) {
        Ok(response) => (
            StatusCode::OK,
            Json(ApiResponse::ok(TtsTestResponse {
                provider: format!("{:?}", state.config.runtime_config().tts.provider),
                latency_ms: start.elapsed().as_millis() as u64,
                audio_format: response.audio_format,
                audio_base64: Some(STANDARD.encode(&response.audio_bytes)),
            })),
        ),
        Err(err) => (StatusCode::BAD_GATEWAY, Json(ApiResponse::<TtsTestResponse>::error("PROVIDER_TEST_FAILED", &err.to_string()))),
    }
}

/// POST /api/test/stt - Test the active STT provider.
pub async fn test_stt(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<TestResponse>>) {
    let state = state.lock().await;
    let provider = match state.providers.stt_provider(&state.config) {
        Ok(provider) => provider,
        Err(err) => return (StatusCode::BAD_REQUEST, Json(ApiResponse::<TestResponse>::error("PROVIDER_NOT_CONFIGURED", &err.to_string()))),
    };

    let audio_bytes = vec![0_u8; 32_000];
    let start = Instant::now();
    match provider.transcribe(SttRequest {
        audio_format: "pcm16".to_string(),
        sample_rate: 16_000,
        channels: 1,
        audio_bytes,
        language: Some("auto".to_string()),
    }) {
        Ok(response) => (
            StatusCode::OK,
            Json(ApiResponse::ok(TestResponse {
                provider: "whisper".to_string(),
                success: true,
                message: response.transcript,
                latency_ms: Some(start.elapsed().as_millis() as u64),
            })),
        ),
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            Json(ApiResponse::<TestResponse>::error("PROVIDER_TEST_FAILED", &err.to_string())),
        ),
    }
}

/// POST /api/test/vts - Test VTS frontend connection.
pub async fn test_vts(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<TestResponse>>) {
    let state = state.lock().await;
    let adapter = state.providers.vts_adapter(&state.config);

    let start = Instant::now();
    match adapter.send_output(voxalive_providers::frontend::FrontendOutput {
        request_id: "vts-test-001".to_string(),
        text: Some("VoxaLive VTS smoke test".to_string()),
        audio_format: None,
        audio_bytes: None,
    }) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::ok(TestResponse {
                provider: "vts".to_string(),
                success: true,
                message: "VTS test endpoint ready".to_string(),
                latency_ms: Some(start.elapsed().as_millis() as u64),
            })),
        ),
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            Json(ApiResponse::<TestResponse>::error("PROVIDER_TEST_FAILED", &err.to_string()))),
    }
}
