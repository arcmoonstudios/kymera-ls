//! Error types shared across the Kymera ecosystem.

use thiserror::Error;

/// Core error type for the Kymera ecosystem
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Type error: {0}")]
    Type(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>; 