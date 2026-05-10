//! LLM provider adapters.
//!
//! Implements LlmProvider port trait.
//! Each adapter implements this trait.

pub mod domain;
pub mod adapters;

pub use domain::{LlmRequest, LlmResponse};

use async_trait::async_trait;
use voxalive_core::domain::CoreError;

/// LLM provider port trait.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, CoreError>;
}