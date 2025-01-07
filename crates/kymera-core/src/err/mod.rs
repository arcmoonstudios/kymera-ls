//! Error types for the Kymera core.
//! 
//! This module provides foundational error types that are used across
//! the Kymera system. It establishes patterns for error handling that
//! other modules build upon.

use thiserror::Error as CoreError;

/// Core error type that serves as the foundation for more specific errors
#[derive(Debug, CoreError)]
pub enum CoreError {
    #[error("Compilation error: {message}")]
    Compilation {
        message: String,
    },

    #[error("Runtime error: {message}")]
    Runtime {
        message: String,
    },

    #[error("GPU error: {message}")]
    Gpu {
        message: String,
    },

    #[error("Configuration error: {message}")]
    Config {
        message: String,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {message}")]
    Internal {
        message: String,
    },
}

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, CoreError>;

impl CoreError {
    /// Creates a new compilation error
    pub fn compilation_error(message: impl Into<String>) -> Self {
        Self::Compilation {
            message: message.into(),
        }
    }

    /// Creates a new runtime error
    pub fn runtime_error(message: impl Into<String>) -> Self {
        Self::Runtime {
            message: message.into(),
        }
    }

    /// Creates a new GPU error
    pub fn gpu_error(message: impl Into<String>) -> Self {
        Self::Gpu {
            message: message.into(),
        }
    }

    /// Creates a new configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Creates a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal {
            message: msg.into(),
        }
    }

    /// Returns the error message
    pub fn message(&self) -> String {
        self.to_string()
    }
} 