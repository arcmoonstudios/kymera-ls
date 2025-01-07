//! Error types for the Kymera analysis.

use thiserror::Error;
use kymera_parser::position::Span;

/// Analysis-specific error type
#[derive(Debug, Error)]
pub enum Error {
    #[error("Type error at {span:?}: {message}")]
    Type {
        span: Span,
        message: String,
    },

    #[error("Name resolution error at {span:?}: {message}")]
    NameResolution {
        span: Span,
        message: String,
    },

    #[error("Semantic error at {span:?}: {message}")]
    Semantic {
        span: Span,
        message: String,
    },

    #[error("Validation error at {span:?}: {message}")]
    Validation {
        span: Span,
        message: String,
    },

    #[error("Parser error: {0}")]
    Parser(#[from] Box<kymera_parser::Error>),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>; 