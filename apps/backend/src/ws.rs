//! WebSocket handler for unified endpoint.
//!
//! Implements /ws/unified with frontend adapter support.
//! See docs/08_WS_PROTOCOL.md for the full protocol spec.

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    extract::ws::{WebSocketUpgrade, WebSocket},
};
use serde::Deserialize;
use tracing::info;

use voxalive_protocol::ws::{
    ClientMessage, ConnectionReady, InputAudioEnd, InputAudioStart, InputText,
    ResponseText, ServerMessage, WebSocketError,
    PROTOCOL_VERSION,
};

use crate::state::AppState;

/// Query parameters for WS connection.
#[derive(Debug, Deserialize)]
pub struct WsQuery {
    pub frontend: Option<String>,
}

/// Handle WebSocket upgrade.
pub async fn ws_handler(
    Query(query): Query<WsQuery>,
    State(state): State<Arc<Mutex<AppState>>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let frontend = query.frontend.unwrap_or_else(|| "vts".to_string());

    ws.on_upgrade(move |socket| handle_socket(socket, state, frontend))
}

/// Handle a WebSocket connection.
async fn handle_socket(
    mut socket: WebSocket,
    _state: Arc<Mutex<AppState>>,
    frontend: String,
) {
    use axum::extract::ws::Message;

    info!("WS connected, frontend={}", frontend);
    let session_id = uuid::Uuid::new_v4().to_string();

    // Send connection.ready
    let ready = ServerMessage::ConnectionReady(ConnectionReady {
        v: PROTOCOL_VERSION,
        frontend: frontend.clone(),
        session_id: session_id.clone(),
    });
    if let Ok(json) = serde_json::to_string(&ready) {
        let _ = socket.send(Message::Text(json.into())).await;
    }

    // TODO: Wire to provider pipeline:
    // - On InputText: send to LLM → TTS → VTS adapter
    // - On InputAudioStart/Chunk/End: buffer → STT → LLM → TTS → VTS
    // - Stream responses back as ServerMessage frames

    // Handle incoming messages
    let mut socket = socket;
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Try to parse as ClientMessage
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::InputText(InputText { request_id, text, .. })) => {
                        info!("WS input.text: {}", text);
                        // Placeholder response
                        let response = ServerMessage::ResponseText(ResponseText {
                            v: PROTOCOL_VERSION,
                            request_id,
                            text: "Echo: ".to_string() + &text,
                        });
                        if let Ok(json) = serde_json::to_string(&response) {
                            let _ = socket.send(Message::Text(json.into())).await;
                        }
                    }
                    Ok(ClientMessage::InputAudioStart(InputAudioStart { request_id, .. })) => {
                        info!("WS input.audio.start: {}", request_id);
                    }
                    Ok(ClientMessage::InputAudioEnd(InputAudioEnd { request_id, .. })) => {
                        info!("WS input.audio.end: {}", request_id);
                    }
                    Err(e) => {
                        let error = ServerMessage::Error(WebSocketError {
                            v: PROTOCOL_VERSION,
                            request_id: None,
                            code: "PROTOCOL_ERROR".to_string(),
                            message: format!("Invalid message: {}", e),
                        });
                        if let Ok(json) = serde_json::to_string(&error) {
                            let _ = socket.send(Message::Text(json.into())).await;
                        }
                    }
                }
            }
            Ok(Message::Binary(_)) => {
                // TODO: buffer audio chunks
            }
            Ok(Message::Close(_)) => {
                info!("WS closed");
                break;
            }
            Err(e) => {
                info!("WS error: {}", e);
                break;
            }
            _ => {}
        }
    }

    info!("WS disconnected, session_id={}", session_id);
}
