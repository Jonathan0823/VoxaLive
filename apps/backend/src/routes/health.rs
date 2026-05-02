//! Health check endpoint.

use axum::{http::StatusCode, Json};
use voxalive_protocol::api::{ApiResponse, HealthResponse};

/// GET /api/health - Backend health check.
pub async fn health() -> (StatusCode, Json<ApiResponse<HealthResponse>>) {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: "0.1.0".to_string(),
        uptime_sec: 0,
        active_llm_provider: "".to_string(),
        active_tts_provider: "".to_string(),
    };
    (StatusCode::OK, Json(ApiResponse::ok(response)))
}