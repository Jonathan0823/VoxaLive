//! Admin authentication middleware.

use axum::{
    body::Body,
    extract::Request,
    http::{ header::HeaderName, HeaderValue, Method, StatusCode },
    middleware::Next,
    response::Response,
};
use std::str::FromStr;

/// Admin token header name.
pub const ADMIN_TOKEN_HEADER: &str = "x-admin-token";

/// Validates admin token from header.
pub async fn admin_auth_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    // Get admin token from header
    let token = request
        .headers()
        .get(ADMIN_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok());

    // TODO: Validate against configured admin token from secrets
    // For MVP, accept any non-empty token
    match token {
        Some(t) if !t.is_empty() => next.run(request).await,
        _ => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap(),
    }
}