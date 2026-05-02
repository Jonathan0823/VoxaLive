//! Config endpoints.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use voxalive_config::models::{LlmProviderKind, SttDevice, TtsProviderKind};
use voxalive_protocol::api::{
    ApiResponse, ConfigData, ConfigPatchRequest, ConfigResponse,
};

use crate::state::AppState;

/// Convert RuntimeConfig to ConfigData via serde.
fn runtime_to_config_data(runtime: &voxalive_config::RuntimeConfig) -> ConfigData {
    serde_json::from_value(serde_json::to_value(runtime).unwrap()).unwrap()
}

/// GET /api/config - Get runtime config.
pub async fn get_config(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<ConfigResponse>>) {
    let state = state.lock().await;
    let data = runtime_to_config_data(state.config.runtime_config());
    (StatusCode::OK, Json(ApiResponse::ok(ConfigResponse { data })))
}

/// PATCH /api/config - Update runtime config (partial).
pub async fn update_config(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(patch): Json<ConfigPatchRequest>,
) -> (StatusCode, Json<ApiResponse<ConfigResponse>>) {
    let mut state = state.lock().await;
    let mut runtime = state.config.runtime_config().clone();

    // Apply partial update
    if let Some(llm) = patch.llm {
        runtime.llm.provider = match llm.provider.as_str() {
            "gemini" => LlmProviderKind::Gemini,
            "openrouter" => LlmProviderKind::OpenRouter,
            "ollama" => LlmProviderKind::Ollama,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        "CONFIG_VALIDATION_FAILED",
                        "Invalid LLM provider",
                    )),
                )
            }
        };
        runtime.llm.model = llm.model;
        runtime.llm.temperature = llm.temperature;
        runtime.llm.max_tokens = llm.max_tokens;
    }
    if let Some(tts) = patch.tts {
        runtime.tts.mode = tts.mode;
        runtime.tts.provider = match tts.provider.as_str() {
            "piper" => TtsProviderKind::Piper,
            "qwen" => TtsProviderKind::Qwen,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        "CONFIG_VALIDATION_FAILED",
                        "Invalid TTS provider",
                    )),
                )
            }
        };
        runtime.tts.model_path = tts.model_path;
    }
    if let Some(stt) = patch.stt {
        runtime.stt.device = match stt.device.as_str() {
            "cpu" => SttDevice::Cpu,
            "cuda:0" => SttDevice::Cuda0,
            "auto" => SttDevice::Auto,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        "CONFIG_VALIDATION_FAILED",
                        "Invalid STT device",
                    )),
                )
            }
        };
    }
    if let Some(live) = patch.live {
        runtime.live.enabled = live.enabled;
        runtime.live.youtube_video_id = live.youtube_video_id;
        runtime.live.tiktok_room = live.tiktok_room;
    }

    match state.config.update_runtime_config(runtime) {
        Ok(_) => {
            let data = runtime_to_config_data(state.config.runtime_config());
            (
                StatusCode::OK,
                Json(ApiResponse::ok(ConfigResponse { data })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("CONFIG_ERROR", "Invalid config")),
        ),
    }
}