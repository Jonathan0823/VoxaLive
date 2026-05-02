//! VoxaLive backend server.

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{middleware, routing::{get, patch, post}, Router};
use tokio::sync::Mutex;
use tracing_subscriber;

use voxalive_config::{ConfigManager, RuntimeConfig};

mod provider_factory;
mod state;
mod routes;
mod admin_middleware;

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

    // Build router with admin auth on protected routes
    let protected = Router::new()
        .route("/api/config", get(routes::config::get_config))
        .route("/api/config", patch(routes::config::update_config))
        .route("/api/secrets/status", get(routes::secrets::get_secret_status))
        .route("/api/test/llm", post(routes::test::test_llm))
        .route("/api/test/tts", post(routes::test::test_tts))
        .route("/api/test/stt", post(routes::test::test_stt))
        .route("/api/test/vts", post(routes::test::test_vts))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware::admin_auth_middleware,
        ));

    let app = Router::new()
        .route("/api/health", get(routes::health::health))
        .merge(protected)
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
