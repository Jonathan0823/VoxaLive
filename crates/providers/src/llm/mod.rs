//! LLM provider adapters.
//!
//! Implements LlmProvider port trait.
//! Each adapter implements this trait.

pub mod domain;
pub mod adapters;

pub use domain::{LlmRequest, LlmResponse};

use voxalive_core::domain::CoreError;

/// LLM provider port trait.
pub trait LlmProvider {
    fn generate(&self, request: LlmRequest) -> Result<LlmResponse, CoreError>;
}