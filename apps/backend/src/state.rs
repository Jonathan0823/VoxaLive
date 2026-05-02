//! Application state for Axum handlers.

use voxalive_config::ConfigManager;
use crate::provider_factory::ProviderFactory;

/// Application state shared across all handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: ConfigManager,
    pub providers: ProviderFactory,
}

impl AppState {
    pub fn new(config: ConfigManager) -> Self {
        Self {
            config,
            providers: ProviderFactory::default(),
        }
    }
}
