//! TTS adapter implementations.

pub mod audio_service;
pub mod piper;

pub use audio_service::AudioServiceTtsAdapter;
pub use piper::PiperAdapter;
