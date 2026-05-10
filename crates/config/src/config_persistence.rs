//! Runtime config persistence.

use std::{fs, path::PathBuf};

const DEFAULT_CONFIG_PATH: &str = "data/config.runtime.json";

/// Resolve runtime config file path from env or default.
pub fn runtime_config_path() -> PathBuf {
    std::env::var("CONFIG_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_CONFIG_PATH))
}

/// Load raw runtime config JSON from disk.
pub fn load(path: &PathBuf) -> Option<String> {
    fs::read_to_string(path).ok()
}

/// Save raw runtime config JSON to disk.
pub fn save(path: &PathBuf, data: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, data)
}
