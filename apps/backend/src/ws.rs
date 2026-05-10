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
    ClientMessage, ConnectionReady,
    ResponseText, ServerMessage, WebSocketError,
    PROTOCOL_VERSION,
};
use voxalive_providers::frontend::{FrontendAdapter, FrontendOutput};

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
    state: Arc<Mutex<AppState>>,
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

    // Handle incoming messages
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::InputText(input)) => {
                        info!("WS input.text: {}", input.text);
                        
                        // Get providers from state (brief lock)
                        let (llm_provider, tts_provider, vts_adapter) = {
                            let state = state.lock().await;
                            let llm = state.providers.llm_provider(&state.config).await.ok();
                            let tts = state.providers.tts_provider(&state.config).ok();
                            let vts = state.providers.vts_adapter(&state.config);
                            (llm, tts, vts)
                        };

                        // Process with providers asynchronously
                        // LLM is async, TTS is still blocking
                        let result: Option<(String, Vec<u8>, String)> = async {
                            let llm_response = match llm_provider {
                                Some(provider) => provider.generate(voxalive_providers::llm::domain::LlmRequest {
                                    prompt: input.text.clone(),
                                    provider: None,
                                }).await.ok(),
                                None => None,
                            }?;

                            let tts_response = match tts_provider {
                                Some(provider) => {
                                    let text = llm_response.text.clone();
                                    tokio::task::spawn_blocking(move || {
                                        provider.synthesize(voxalive_providers::tts::domain::TtsRequest {
                                            text,
                                            provider: None,
                                        })
                                    }).await.ok().and_then(|r| r.ok())
                                }
                                None => None,
                            }?;

                            Some((llm_response.text, tts_response.audio_bytes, tts_response.audio_format))
                        }.await;
                        
                        // Send audio to VTS adapter
                        if let Some((ref _text, ref audio_bytes, ref audio_format)) = result {
                            let output = FrontendOutput {
                                request_id: input.request_id.clone(),
                                text: None,
                                audio_format: Some(audio_format.clone()),
                                audio_bytes: Some(audio_bytes.clone()),
                            };
                            let _ = vts_adapter.send_output(output);
                        }
                        
                        // Send text response back to client
                        let response = ServerMessage::ResponseText(ResponseText {
                            v: PROTOCOL_VERSION,
                            request_id: input.request_id,
                            text: result
                                .map(|(text, _, _)| text)
                                .unwrap_or_else(|| "Error processing request".to_string()),
                        });
                        if let Ok(json) = serde_json::to_string(&response) {
                            let _ = socket.send(Message::Text(json.into())).await;
                        }
                    }
                    Ok(ClientMessage::InputAudioStart(start)) => {
                        info!("WS input.audio.start: {}", start.request_id);
                        // TODO: Start buffering audio
                    }
                    Ok(ClientMessage::InputAudioEnd(end)) => {
                        info!("WS input.audio.end: {}", end.request_id);
                        // TODO: Process buffered audio through STT → LLM → TTS → VTS
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
                // TODO: Buffer audio chunks for audio input
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
