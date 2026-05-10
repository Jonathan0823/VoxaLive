//! VoxaLive config crate.
//!
//! Contains config loading, runtime config, validation, and secret handling.

use std::collections::HashMap;
use std::path::PathBuf;

use thiserror::Error;

pub mod models;
pub mod secret_persistence;
pub mod secrets;

pub use models::{
    validate_config, LiveConfig, LlmConfig, RuntimeConfig, ServerConfig, SttConfig, TtsConfig,
};
pub use secrets::{
    create_secret_status, is_configured, SecretStatus, SecretStatusResponse, SecretUpdateRequest,
    SecretUpdateResponse, SECRET_KEYS,
};

/// Errors raised by config operations.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("validation failed: {0}")]
    Validation(String),
}

/// Owns the runtime config and validates updates.
#[derive(Debug, Clone)]
pub struct ConfigManager {
    runtime: RuntimeConfig,
    secrets: SecretManager,
    secrets_path: PathBuf,
}

impl ConfigManager {
    pub fn new(runtime: RuntimeConfig) -> Result<Self, ConfigError> {
        validate_config(&runtime).map_err(ConfigError::Validation)?;
        Ok(Self {
            runtime,
            secrets: SecretManager::new(),
            secrets_path: secret_persistence::secrets_path(),
        })
    }

    pub fn default() -> Self {
        Self {
            runtime: RuntimeConfig::default(),
            secrets: SecretManager::new(),
            secrets_path: secret_persistence::secrets_path(),
        }
    }

    pub fn runtime_config(&self) -> &RuntimeConfig {
        &self.runtime
    }

    pub fn secret_status(&self) -> SecretStatusResponse {
        self.secrets.status()
    }

    pub fn secret_value(&self, key: &str) -> Option<String> {
        self.secrets.get(key)
    }

    pub fn update_secrets(&mut self, request: SecretUpdateRequest) -> SecretUpdateResponse {
        self.secrets.update(request)
    }

    pub fn update_runtime_config(&mut self, next: RuntimeConfig) -> Result<(), ConfigError> {
        validate_config(&next).map_err(ConfigError::Validation)?;
        self.runtime = next;
        Ok(())
    }

    /// Get config as JSON string.
    pub fn load(&self) -> String {
        serde_json::to_string(&self.runtime).unwrap_or_default()
    }

    /// Update config from JSON string.
    pub fn save(&mut self, data: String) -> Result<(), ConfigError> {
        let next: RuntimeConfig = serde_json::from_str(&data)
            .map_err(|e| ConfigError::Validation(e.to_string()))?;
        self.update_runtime_config(next)
    }

    /// Get secret status.
    pub fn get_status(&self) -> String {
        serde_json::to_string(&self.secrets.status()).unwrap_or_default()
    }

    /// Set admin token from environment (called at startup).
    pub fn set_admin_token_from_env(&mut self) {
        if let Ok(token) = std::env::var("ADMIN_TOKEN") {
            if !token.is_empty() {
                self.secrets.update(SecretUpdateRequest {
                    admin_token: Some(token),
                    gemini_api_key: None,
                    openrouter_api_key: None,
                    vts_auth_token: None,
                });
            }
        }
    }

    /// Load all bootstrap secrets from environment variables.
    /// Env values take precedence over any previously loaded secrets.
    /// Persists merged secrets to disk so runtime updates survive restarts.
    pub fn load_secrets_from_env(&mut self) {
        // First try to load persisted secrets
        let persisted = secret_persistence::load(&self.secrets_path);
        if let Some(file) = persisted {
            for (key, value) in file.secrets {
                if !value.is_empty() {
                    self.secrets.secrets.insert(key, value);
                }
            }
        }

        // Env values override persisted (for bootstrap rotation)
        let secrets = SecretUpdateRequest {
            admin_token: std::env::var("ADMIN_TOKEN").ok().filter(|v| !v.is_empty()),
            gemini_api_key: std::env::var("GEMINI_API_KEY").ok().filter(|v| !v.is_empty()),
            openrouter_api_key: std::env::var("OPENROUTER_API_KEY").ok().filter(|v| !v.is_empty()),
            vts_auth_token: std::env::var("VTS_AUTH_TOKEN").ok().filter(|v| !v.is_empty()),
        };
        self.secrets.update(secrets);

        // Persist current merged state
        let _ = self.persist_secrets();
    }

    /// Persist current secrets to encrypted file.
    pub fn persist_secrets(&self) -> Result<(), ConfigError> {
        secret_persistence::save(&self.secrets_path, &self.secrets.secrets)
            .map_err(|e| ConfigError::Validation(format!("failed to persist secrets: {}", e)))
    }
}

/// Owns secret values and returns safe status only.
#[derive(Debug, Clone, Default)]
pub struct SecretManager {
    secrets: HashMap<String, String>,
}

impl SecretManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&self) -> SecretStatusResponse {
        let secrets = SECRET_KEYS
            .iter()
            .map(|key| {
                let configured = self.secrets.get(*key).is_some_and(|value| !value.is_empty());
                ((*key).to_string(), create_secret_status(configured))
            })
            .collect();

        SecretStatusResponse { secrets }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.secrets.get(key).cloned()
    }

    pub fn has(&self, key: &str) -> bool {
        self.secrets.get(key).is_some_and(|value| !value.is_empty())
    }

    pub fn update(&mut self, request: SecretUpdateRequest) -> SecretUpdateResponse {
        let mut updated = Vec::new();

        self.apply_secret("gemini_api_key", request.gemini_api_key, &mut updated);
        self.apply_secret("openrouter_api_key", request.openrouter_api_key, &mut updated);
        self.apply_secret("admin_token", request.admin_token, &mut updated);
        self.apply_secret("vts_auth_token", request.vts_auth_token, &mut updated);

        SecretUpdateResponse { updated }
    }

    fn apply_secret(
        &mut self,
        key: &str,
        value: Option<String>,
        updated: &mut Vec<String>,
    ) {
        if let Some(value) = value {
            self.secrets.insert(key.to_string(), value);
            updated.push(key.to_string());
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self {
            runtime: RuntimeConfig::default(),
            secrets: SecretManager::new(),
            secrets_path: secret_persistence::secrets_path(),
        }
    }
}
