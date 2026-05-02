//! Application state for Axum handlers.

use voxalive_config::ConfigManager;

/// Application state shared across all handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: ConfigManager,
}

impl AppState {
    pub fn new(config: ConfigManager) -> Self {
        Self { config }
    }
}