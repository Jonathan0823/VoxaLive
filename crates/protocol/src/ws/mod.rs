//! WebSocket message DTOs.
//!
//! All WebSocket protocol types should live in this module.
//! See docs/08_WS_PROTOCOL.md for the full protocol spec.
//!
//! Protocol version: 1 (initial)

use serde::{Deserialize, Serialize};

/// Protocol version constant.
pub const PROTOCOL_VERSION: u32 = 1;

// ========== Client to Server ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    #[serde(rename = "input.text")]
    InputText(InputText),
    #[serde(rename = "input.audio.start")]
    InputAudioStart(InputAudioStart),
    #[serde(rename = "input.audio.end")]
    InputAudioEnd(InputAudioEnd),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputText {
    pub v: u32,
    pub request_id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAudioStart {
    pub v: u32,
    pub request_id: String,
    pub format: String,
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAudioEnd {
    pub v: u32,
    pub request_id: String,
}

// ========== Server to Client ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    #[serde(rename = "connection.ready")]
    ConnectionReady(ConnectionReady),
    #[serde(rename = "response.text")]
    ResponseText(ResponseText),
    #[serde(rename = "response.audio.start")]
    ResponseAudioStart(ResponseAudioStart),
    #[serde(rename = "response.audio.end")]
    ResponseAudioEnd(ResponseAudioEnd),
    #[serde(rename = "response.visemes")]
    ResponseVisemes(ResponseVisemes),
    #[serde(rename = "error")]
    Error(WebSocketError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionReady {
    pub v: u32,
    pub frontend: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseText {
    pub v: u32,
    pub request_id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAudioStart {
    pub v: u32,
    pub request_id: String,
    pub format: String,
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAudioEnd {
    pub v: u32,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viseme {
    pub time_ms: u32,
    pub value: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseVisemes {
    pub v: u32,
    pub request_id: String,
    pub visemes: Vec<Viseme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketError {
    pub v: u32,
    pub request_id: Option<String>,
    pub code: String,
    pub message: String,
}
