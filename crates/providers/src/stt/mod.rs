//! STT provider adapters.
//!
//! Implements SttProvider port trait.
//! Each adapter implements this trait.

pub mod domain;
pub mod adapters;

pub use domain::{SttRequest, SttResponse};

use voxalive_core::domain::CoreError;

/// STT provider port trait.
pub trait SttProvider: Send {
    fn transcribe(&self, request: SttRequest) -> Result<SttResponse, CoreError>;
}