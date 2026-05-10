//! Secret management endpoints.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use voxalive_config::SecretUpdateRequest;
use voxalive_protocol::api::{ApiResponse, SecretStatusResponse, SecretUpdateResponse as ProtocolSecretUpdateResponse};

use crate::state::AppState;

/// GET /api/secrets/status - Get secret status (not values).
pub async fn get_secret_status(
    State(state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<SecretStatusResponse>>) {
    let state = state.lock().await;
    let config_status = state.config.secret_status();
    let status: SecretStatusResponse =
        serde_json::from_value(serde_json::to_value(&config_status).unwrap()).unwrap();
    (StatusCode::OK, Json(ApiResponse::ok(status)))
}

/// PUT /api/secrets - Update secrets and persist to disk.
pub async fn put_secrets(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(secrets): Json<SecretUpdateRequest>,
) -> (StatusCode, Json<ApiResponse<ProtocolSecretUpdateResponse>>) {
    let mut state = state.lock().await;
    let result = state.config.update_secrets(secrets);

    // Persist updated secrets so they survive restarts
    if let Err(e) = state.config.persist_secrets() {
        tracing::warn!("failed to persist secrets: {}", e);
    }

    // Convert to protocol type for API response
    let response: ProtocolSecretUpdateResponse =
        serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();

    (StatusCode::OK, Json(ApiResponse::ok(response)))
}