//! Minimal prelude / compatibility layer for storage crate

pub use anyhow::{Context, Result as AppResult};

/// Create a storage-related error
pub fn storage_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::anyhow!(msg.into())
}

/// Create a configuration-related error
pub fn configuration_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::anyhow!(msg.into())
}

/// Create an internal error
pub fn internal_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::anyhow!(msg.into())
}
