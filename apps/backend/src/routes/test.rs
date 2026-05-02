//! Provider test endpoints.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use voxalive_protocol::api::{ApiResponse, TestResponse};

use crate::state::AppState;

/// POST /api/test/vts - Test VTS frontend connection.
pub async fn test_vts(
    State(_state): State<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<ApiResponse<TestResponse>>) {
    // TODO: Actual VTS connection test
    let response = TestResponse {
        provider: "vts".to_string(),
        success: true,
        message: "VTS test endpoint ready".to_string(),
    };
    (StatusCode::OK, Json(ApiResponse::ok(response)))
}