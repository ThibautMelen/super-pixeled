//! Error types for Super Pixeled.

use thiserror::Error;

/// Result type alias using our Error.
pub type Result<T> = std::result::Result<T, Error>;

/// Unified error type for the application.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Invalid parameter: {field} - {message}")]
    InvalidParameter { field: String, message: String },

    #[error("Effect not found: {0}")]
    EffectNotFound(String),

    #[error("Hardware error: {0}")]
    Hardware(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Create an invalid parameter error.
    pub fn invalid_param(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidParameter {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a hardware error.
    pub fn hardware(message: impl Into<String>) -> Self {
        Self::Hardware(message.into())
    }
}
