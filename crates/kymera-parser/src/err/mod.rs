//! Error types for the Kymera parser.

pub use thiserror::Error;
use crate::position::Span;

/// Parser-specific error type
#[derive(Debug, Error)]
pub enum KymeraParserError {
    #[error("Lexer error at {span:?}: {message}")]
    Lexer {
        span: Span,
        message: String,
    },

    #[error("Parser error at {span:?}: {message}")]
    Parser {
        span: Span,
        message: String,
    },

    #[error("Syntax error at {span:?}: {message}")]
    Syntax {
        span: Span,
        message: String,
    },

    #[error("Unexpected token at {span:?}: expected {expected}, found {found}")]
    UnexpectedToken {
        span: Span,
        expected: String,
        found: String,
    },

    #[error("Unexpected end of input at {span:?}")]
    UnexpectedEof {
        span: Span,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, KymeraParserError>; 