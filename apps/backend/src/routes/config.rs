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
    if let Some(server) = patch.server {
        if let Some(vts_endpoint) = server.vts_endpoint {
            if vts_endpoint.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        "CONFIG_VALIDATION_FAILED",
                        "VTS endpoint cannot be empty",
                    )),
                );
            }
            // Reject bind addresses - vts_endpoint must be a real host to connect to
            let lower = vts_endpoint.to_lowercase();
            if lower.starts_with("ws://0.0.0.0") || lower.starts_with("ws://[::]") || lower.starts_with("ws://[::1]") {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        "CONFIG_VALIDATION_FAILED",
                        "VTS endpoint must be a real IP/hostname, not a bind address like 0.0.0.0",
                    )),
                );
            }
            if !vts_endpoint.starts_with("ws://") && !vts_endpoint.starts_with("wss://") {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        "CONFIG_VALIDATION_FAILED",
                        "VTS endpoint must start with ws:// or wss://",
                    )),
                );
            }
            runtime.server.vts_endpoint = vts_endpoint;
        }
        if let Some(name) = server.vts_plugin_name {
            runtime.server.vts_plugin_name = name;
        }
        if let Some(dev) = server.vts_plugin_developer {
            runtime.server.vts_plugin_developer = dev;
        }
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