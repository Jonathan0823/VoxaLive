//! Secret handling.
//!
//! SecretManager, secret status, and safe secret updates.
//! See docs/09_CONFIG_AND_SECRETS.md for the full spec.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Secret status response (no raw values).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStatusResponse {
    pub secrets: HashMap<String, SecretStatus>,
}

/// Status of a single secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStatus {
    pub configured: bool,
}

/// Secret update request.
#[derive(Debug, Clone, Deserialize)]
pub struct SecretUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemini_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openrouter_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vts_auth_token: Option<String>,
}

/// Secret update response (no raw values echoed back).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretUpdateResponse {
    pub updated: Vec<String>,
}

/// Secret keys we track.
pub const SECRET_KEYS: &[&str] = &[
    "gemini_api_key",
    "openrouter_api_key",
    "admin_token",
    "vts_auth_token",
];

/// Check if a secret value is configured (non-empty).
pub fn is_configured(value: &Option<String>) -> bool {
    value.as_ref().map_or(false, |v| !v.is_empty())
}

/// Create a safe secret status (never returns raw values).
pub fn create_secret_status(configured: bool) -> SecretStatus {
    SecretStatus { configured }
}
