//! Error types for the Kymera parser.
//! 
//! This module provides specialized error types for lexical analysis,
//! parsing, and syntax validation. It builds on the core error system
//! while adding parser-specific context.

use thiserror::Error;
use anyhow::Result as AnyhowResult;
use crate::position::Span;

/// Parser-specific error type
#[derive(Debug, Error)]
pub enum ParserError {
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

/// Result type alias for parser operations
pub type Result<T> = std::result::Result<T, ParserError>;

// Also expose the specific ParserResult type for new code
pub type ParserResult<T> = AnyhowResult<T>;

impl ParserError {
    /// Creates a new lexer error
    pub fn lexer_error(span: Span, message: impl Into<String>) -> Self {
        Self::Lexer {
            span,
            message: message.into(),
        }
    }

    /// Creates a new parser error
    pub fn parser_error(span: Span, message: impl Into<String>) -> Self {
        Self::Parser {
            span,
            message: message.into(),
        }
    }

    /// Creates a new unexpected token error
    pub fn unexpected_token(span: Span, expected: impl Into<String>, found: impl Into<String>) -> Self {
        Self::UnexpectedToken {
            span,
            expected: expected.into(),
            found: found.into(),
        }
    }

    /// Creates a new unexpected EOF error
    pub fn unexpected_eof(span: Span) -> Self {
        Self::UnexpectedEof { span }
    }

    /// Creates a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Returns the error span if available
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Lexer { span, .. } => Some(*span),
            Self::Parser { span, .. } => Some(*span),
            Self::UnexpectedToken { span, .. } => Some(*span),
            Self::UnexpectedEof { span } => Some(*span),
            _ => None,
        }
    }

    /// Returns the error message
    pub fn message(&self) -> String {
        match self {
            Self::Lexer { message, .. } => message.clone(),
            Self::Parser { message, .. } => message.clone(),
            Self::UnexpectedToken { expected, found, .. } => format!("expected {}, found {}", expected, found),
            Self::UnexpectedEof { .. } => "unexpected end of input".to_string(),
            Self::Io(e) => e.to_string(),
            Self::Internal(msg) => msg.clone(),
        }
    }
} 