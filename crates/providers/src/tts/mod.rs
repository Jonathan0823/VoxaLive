//! TTS provider adapters.
//!
//! Implements TtsProvider port trait.
//! Each adapter implements this trait.

pub mod domain;
pub mod adapters;

pub use domain::{TtsRequest, TtsResponse};

use voxalive_core::domain::CoreError;

/// TTS provider port trait.
pub trait TtsProvider: Send {
    fn synthesize(&self, request: TtsRequest) -> Result<TtsResponse, CoreError>;
}