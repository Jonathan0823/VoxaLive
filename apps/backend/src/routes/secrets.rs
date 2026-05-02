//! Secret management endpoints.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use voxalive_protocol::api::{ApiResponse, SecretResponse};

use crate::state::AppState;

/// GET /api/secrets - Get secret status (not values).
pub async fn get_secrets(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<SecretResponse>>) {
    let state = state.lock().await;
    let status = state.config.get_status();
    (StatusCode::OK, Json(ApiResponse::ok(SecretResponse { status })))
}