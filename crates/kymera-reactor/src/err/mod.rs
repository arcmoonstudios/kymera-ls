//! Error types for the Kymera reactor.

use thiserror::Error;
use kymera_parser::err::ParserError;
use kymera_analysis::err::AnalysisError;

/// Error type for reactor operations
#[derive(Debug, Error)]
pub enum ReactorError {
    /// Parser errors
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    /// Analysis errors
    #[error("Analysis error: {0}")]
    Analysis(#[from] AnalysisError),

    /// Engine errors
    #[error("Engine error: {0}")]
    EngineError(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Module errors
    #[error("Module error: {0}")]
    ModuleError(String),
}

pub type Result<T> = std::result::Result<T, ReactorError>; 