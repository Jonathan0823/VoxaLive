//! VoxaLive core domain logic, ports, and services.
//!
//! This crate contains backend domain types and port traits.
//! It must not depend on Axum, provider SDKs, or frontend implementations.

pub mod domain;
pub mod ports;
pub mod service;