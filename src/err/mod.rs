//! Error types for the Kymera language server.

use thiserror::Error;
use tower_lsp::jsonrpc;

/// Language server error type
#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON-RPC error: {0}")]
    JsonRpc(#[from] jsonrpc::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parser error: {0}")]
    Parser(#[from] kymera_parser::err::Error),

    #[error("Analysis error: {0}")]
    Analysis(#[from] kymera_analysis::err::Error),

    #[error("Reactor error: {0}")]
    Reactor(#[from] kymera_reactor::err::Error),

    #[error("Core error: {0}")]
    Core(#[from] kymera_core::error::Error),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Document error: {0}")]
    Document(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>; 