//! LLM adapter implementations.

pub mod gemini;
pub mod openrouter;
pub mod ollama;

pub use gemini::GeminiAdapter;
pub use ollama::{OllamaAdapter, OllamaMode};
pub use openrouter::OpenRouterAdapter;
