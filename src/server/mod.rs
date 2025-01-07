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
use std::time::Duration;

use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::Client;

use crate::server::{
    capabilities::{build_server_capabilities, CapabilitiesConfig},
    state::{ModuleConfig, MetricsCollector, ServerState},
};

/// The main Kymera Language Server struct.
/// - Holds a `Client` for LSP operations.
/// - Maintains a reference-counted `ServerState` for concurrency-safe data.
/// - Dynamically loads or falls back to minimal LSP capabilities.
pub struct KymeraLanguageServer {
    /// LSP client handle for sending messages to the client.
    pub client: Client,
    /// Shared server state for document management, metrics, etc.
    pub state: Arc<ServerState<String>>,
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
    pub async fn new(client: Client) -> Self {
        // Initialize configuration
        let module_config = ModuleConfig::new()
            .expect("Failed to load server configuration");
            
        // Create a default CapabilitiesConfig
        let capabilities_config = CapabilitiesConfig {
            trigger_characters: vec![],  // Default trigger characters
            language_id: "kymera".to_string(),
            file_scheme: "file".to_string(),
            max_retries: 3,
            load_timeout: Duration::from_secs(5),
        };
            
        // Initialize metrics collector
        let metrics = Arc::new(MetricsCollector::new("kymera_ls".to_string()));

        // Initialize capabilities
        let capabilities = build_server_capabilities(&capabilities_config).await;

        Self {
            client,
            state: Arc::new(ServerState::new(module_config, metrics)),
            capabilities,
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
    pub async fn get_document_content(&self, uri: &str) -> Option<String> {
        self.state.get_document(uri).await.ok()
    }
}
