//! Admin authentication middleware.

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tokio::sync::Mutex;

use crate::state::AppState;

/// Admin token header name.
pub const ADMIN_TOKEN_HEADER: &str = "x-admin-token";

/// Validates admin token from header.
pub async fn admin_auth_middleware(
    State(state): State<Arc<Mutex<AppState>>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let expected_token = {
        let state = state.lock().await;
        state.config.secret_value("admin_token")
    };

    let token = request
        .headers()
        .get(ADMIN_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok());

    match (expected_token, token) {
        (Some(expected), Some(actual)) if !expected.is_empty() && actual == expected => {
            next.run(request).await
        }
        _ => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap(),
    }
}
