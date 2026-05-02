//! VoxaLive providers crate.
//!
//! Contains external provider adapters (LLM, TTS, STT, live inputs, frontend).
//! Each adapter owns its domain types and port trait.

pub mod llm;
pub mod tts;
pub mod stt;
pub mod live;
pub mod frontend;

// Re-export for convenience
pub use voxalive_core::domain::CoreError;