//! src/server/mod.rs
//! LSP server implementation for Kymera, following the Rust Module Enhancement Guide v1.0.
//!
//! # Key Highlights
//! - **Unified Module** that brings together server capabilities, handlers, and state.
//! - **Type-Safe & Concurrent** state management via `DashMap` (see `state`).
//! - **Advanced Error Handling** using custom error enums and `thiserror`.
//! - **Configurable** via environment and JSON files (see `state::ModuleConfig`).
//! - **Retry/Timeout** logic for operations, ensuring resiliency.
//! - **Best Practices** in concurrency, security, observability, and testing.
//! - **Extensible**: optional concurrency features, dynamic capabilities, and more.

/// Core server capabilities module.  
/// Implements dynamic/fallback logic and advanced concurrency features.
pub mod capabilities;

/// LSP request/notification handlers module.
/// Implements the `LanguageServer` trait using `tower_lsp`.
mod handlers;

/// Global server state module.
/// Manages documents, configuration, metrics, and error handling.
mod state;

// -----------------------------------------------------------------------------
// Public Re-Exports
// -----------------------------------------------------------------------------
use std::sync::Arc;

use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::Client;

// Publicly re-export the `ServerState` type for easy consumption.
pub use state::ServerState;

/// The main Kymera Language Server struct.
/// - Holds a `Client` for LSP operations.
/// - Maintains a reference-counted `ServerState` for concurrency-safe data.
/// - Dynamically loads or falls back to minimal LSP capabilities.
pub struct KymeraLanguageServer {
    /// LSP client handle for sending messages to the client.
    pub client: Client,
    /// Shared server state for document management, metrics, etc.
    pub state: Arc<ServerState>,
    /// Cached LSP server capabilities, loaded dynamically or via fallback.
    pub capabilities: ServerCapabilities,
}

impl KymeraLanguageServer {
    /// Creates a new `KymeraLanguageServer` with default server state and capabilities.
    ///
    /// # Examples
    /// ```ignore
    /// let client = tower_lsp::Client::new(...);
    /// let server = KymeraLanguageServer::new(client);
    /// ```
    pub fn new(client: Client) -> Self {
        // Initialize configuration
        let config = ModuleConfig::new()
            .expect("Failed to load server configuration");
            
        // Initialize metrics collector
        let metrics = Arc::new(MetricsCollector::new("kymera_ls".to_string()));

        Self {
            client,
            state: Arc::new(ServerState::new(config, metrics)),
            capabilities: capabilities::server_capabilities(),
        }
    }

    /// Returns a reference to the server's capabilities.
    pub fn get_capabilities(&self) -> &ServerCapabilities {
        &self.capabilities
    }

    /// Returns the text content of a document by URI, if it exists in the server state.
    ///
    /// # Arguments
    /// * `uri` - The URI of the document.
    ///
    /// # Returns
    /// * `Option<String>` - The document content, if found.
    pub fn get_document_content(&self, uri: &str) -> Option<String> {
        self.state.get_document(uri)
    }
}
