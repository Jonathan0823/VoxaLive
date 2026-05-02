//! Config endpoints.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use voxalive_protocol::api::{ApiResponse, ConfigResponse, ConfigUpdateRequest};

use crate::state::AppState;

/// GET /api/config - Get runtime config.
pub async fn get_config(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<ConfigResponse>>) {
    let state = state.lock().await;
    let data = state.config.load();
    (StatusCode::OK, Json(ApiResponse::ok(ConfigResponse { data })))
}

/// PATCH /api/config - Update runtime config.
pub async fn update_config(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ConfigUpdateRequest>,
) -> (StatusCode, Json<ApiResponse<ConfigResponse>>) {
    let mut state = state.lock().await;
    match state.config.save(payload.data) {
        Ok(_) => {
            let data = state.config.load();
            (StatusCode::OK, Json(ApiResponse::ok(ConfigResponse { data })))
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("CONFIG_ERROR", "Invalid config")),
        ),
    }
}