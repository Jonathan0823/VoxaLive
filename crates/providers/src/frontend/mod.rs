//! Frontend output adapters.
//!
//! Implements FrontendAdapter port from crates/core.
//!
//! MVP scope: VTube Studio only.
//!
//! Future reserved adapters (not implemented in MVP):
//! - Raw API adapter
//! - Web3D adapter

use voxalive_core::domain::CoreError;

pub mod vts;

/// Frontend output payload.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FrontendOutput {
    pub request_id: String,
    pub text: Option<String>,
    pub audio_format: Option<String>,
    pub audio_bytes: Option<Vec<u8>>,
}

/// Frontend adapter port trait.
pub trait FrontendAdapter {
    fn send_output(&self, output: FrontendOutput) -> Result<(), CoreError>;
}

// Reserved for future work:
// pub mod raw;   // raw API frontend
// pub mod web3d; // Web3D frontend
