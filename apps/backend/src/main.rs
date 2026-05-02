//! VoxaLive backend server.

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, routing::{get, patch}};
use tokio::sync::Mutex;
use tracing_subscriber;

use voxalive_config::{ConfigManager, RuntimeConfig};

mod state;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Starting VoxaLive backend...");

    // Initialize config
    let runtime = RuntimeConfig::default();
    let config = ConfigManager::new(runtime).unwrap_or_else(|_| ConfigManager::default());
    let state = Arc::new(Mutex::new(state::AppState::new(config)));

    // Build router
    let app = Router::new()
        .route("/api/health", get(routes::health::health))
        .route("/api/config", get(routes::config::get_config))
        .route("/api/config", patch(routes::config::update_config))
        .route("/api/secrets", get(routes::secrets::get_secrets))
        .route("/api/test/vts", get(routes::test::test_vts))
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}