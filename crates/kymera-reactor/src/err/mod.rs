//! Error types for the Kymera reactor.

use thiserror::Error;

/// Compilation error type
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Parsing error: {0}")]
    Parse(String),

    #[error("Type error: {0}")]
    Type(String),

    #[error("Code generation error: {0}")]
    CodeGen(String),

    #[error("Optimization error: {0}")]
    Optimization(String),
}

/// Runtime error type
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Resource error: {0}")]
    Resource(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// GPU acceleration error type
#[derive(Debug, Error)]
pub enum GPUError {
    #[error("Device error: {0}")]
    Device(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Kernel error: {0}")]
    Kernel(String),

    #[error("Synchronization error: {0}")]
    Sync(String),
}

/// Main error type for the Kymera reactor
#[derive(Debug, Error)]
pub enum Error {
    #[error("Compilation error: {0}")]
    Compile(#[from] CompileError),

    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeError),

    #[error("GPU error: {0}")]
    GPU(#[from] GPUError),

    #[error("Parser error: {0}")]
    Parser(#[from] kymera_parser::err::Error),

    #[error("Analysis error: {0}")]
    Analysis(#[from] kymera_analysis::err::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>; 