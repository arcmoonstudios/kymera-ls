//! src/server_state.rs
//! Enhanced Server State module meeting Rust Module Enhancement Guide v1.0
//!
//! # Key Highlights
//! - **Concurrent** document storage via `DashMap`
//! - **Advanced Error Handling** with retry and timeout
//! - **Performance** metrics through `metrics` crate
//! - **Production-Grade** design aligned with best practices
//! - **Configuration** loading via `config` crate
//! - **Extensive Testing** with property-based and scenario-driven tests

use dashmap::DashMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use config::{Config, ConfigError, Environment, File};
use metrics::{counter, histogram};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{Notify, RwLock};
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};

/// Specialized result type for server state operations.
pub type ServerStateResult<T> = Result<T, ServerStateError>;

/// Represents the possible errors that can occur within the server state module.
#[derive(Debug, Error)]
pub enum ServerStateError {
    #[error("Document with URI '{0}' not found")]
    DocumentNotFound(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Operation timed out after {duration:?}")]
    Timeout {
        duration: Duration,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Invalid input: {message}")]
    ValidationError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Operation failed: {message}")]
    OperationError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        retry_count: u32,
    },
}

impl ServerStateError {
    /// Indicates if the error is retryable (e.g., timeouts).
    pub fn is_retryable(&self) -> bool {
        matches!(self, ServerStateError::Timeout { .. })
    }

    /// Returns a string representation of the error type.
    pub fn type_name(&self) -> &'static str {
        match self {
            ServerStateError::DocumentNotFound(_) => "DocumentNotFound",
            ServerStateError::ConfigError(_) => "ConfigError",
            ServerStateError::Timeout { .. } => "Timeout",
            ServerStateError::ValidationError { .. } => "ValidationError",
            ServerStateError::OperationError { .. } => "OperationError",
        }
    }
}

/// Configuration for the module, adhering to best practices.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleConfig {
    /// Maximum number of documents that can be stored.
    #[serde(default = "default_max_documents")]
    pub max_documents: usize,

    /// Timeout for requests in seconds.
    #[serde(with = "humantime_serde", default = "default_request_timeout")]
    pub request_timeout: Duration,

    // Extend with more fields as necessary, e.g. feature flags, logging levels, etc.
}

fn default_max_documents() -> usize {
    100
}

fn default_request_timeout() -> Duration {
    Duration::from_secs(30)
}

impl ModuleConfig {
    /// Creates a new configuration instance, merging from files and environment variables.
    pub fn new() -> Result<Arc<Self>, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        Ok(Arc::new(config.try_deserialize()?))
    }
}

/// Metrics collector for the server state, following the Rust Module Enhancement Guide.
#[derive(Debug)]
pub struct MetricsCollector {
    prefix: String,
}

impl MetricsCollector {
    /// Creates a new metrics collector with the given prefix.
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    /// Records an operation's duration and increments its counter.
    #[instrument(skip(self, name))]
    pub fn record_operation(&self, name: &str, duration: Duration) {
        let duration_secs = duration.as_secs_f64();
        histogram!(
            format!("{}_{}_duration_seconds", self.prefix, name),
            duration_secs,
            "operation" => name
        );
        counter!(
            format!("{}_{}_total", self.prefix, name),
            1,
            "operation" => name
        );
        debug!("Operation '{name}' took {duration_secs:.4} seconds");
    }

    /// Records an error occurrence.
    #[instrument(skip(self, error))]
    pub fn record_error(&self, error: &ServerStateError) {
        counter!(
            format!("{}_errors_total", self.prefix),
            1,
            "error_type" => error.type_name()
        );
        error!("An error occurred: {:?}", error);
    }
}

/// Represents the state of the language server, including document storage, configuration, etc.
#[derive(Debug)]
pub struct ServerState<T: Clone + fmt::Debug + Send + Sync> {
    /// Thread-safe map of document URIs to their content.
    documents: Arc<DashMap<String, T>>,
    /// Notifier for state changes.
    notify: Arc<Notify>,
    /// Aggregates performance and error metrics.
    metrics: Arc<MetricsCollector>,
    /// Configuration data for the server.
    config: Arc<ModuleConfig>,
    /// Optionally track any in-flight operations or concurrency controls.
    _ops_lock: Arc<RwLock<()>>,
}

impl<T: Clone + fmt::Debug + Send + Sync> ServerState<T> {
    /// Constructs a new `ServerState` with the provided configuration and metrics.
    pub fn new(config: Arc<ModuleConfig>, metrics: Arc<MetricsCollector>) -> Self {
        Self {
            documents: Arc::new(DashMap::new()),
            notify: Arc::new(Notify::new()),
            metrics,
            config,
            _ops_lock: Arc::new(RwLock::new(())),
        }
    }

    /// Retrieves the document content by URI, if it exists.
    /// Returns `DocumentNotFound` error if the document is missing.
    #[instrument(skip(self))]
    pub async fn get_document(&self, uri: &str) -> ServerStateResult<T> {
        let start = Instant::now();
        let result = self
            .documents
            .get(uri)
            .map(|doc| doc.value().clone())
            .ok_or_else(|| ServerStateError::DocumentNotFound(uri.to_string()));

        self.metrics.record_operation("get_document", start.elapsed());
        result
    }

    /// Inserts or updates a document in the map, then notifies all waiters.
    #[instrument(skip(self, content))]
    pub fn update_document(&self, uri: String, content: T) {
        let start = Instant::now();
        self.documents.insert(uri.clone(), content);
        self.metrics.record_operation("update_document", start.elapsed());
        info!("Document updated: {uri}");
        self.notify.notify_waiters();
    }

    /// Deletes a document by URI, returning an error if not found.
    #[instrument(skip(self))]
    pub fn delete_document(&self, uri: &str) -> ServerStateResult<()> {
        let start = Instant::now();
        if self.documents.remove(uri).is_none() {
            let error = ServerStateError::DocumentNotFound(uri.to_string());
            self.metrics.record_error(&error);
            return Err(error);
        }
        self.metrics.record_operation("delete_document", start.elapsed());
        info!("Document deleted: {uri}");
        self.notify.notify_waiters();
        Ok(())
    }

    /// Returns an `Arc<Notify>` that can be awaited to detect state changes.
    pub fn notifier(&self) -> Arc<Notify> {
        self.notify.clone()
    }

    /// Executes an async operation with automatic retries and timeout handling.
    /// Retries occur only if the error is considered retryable (e.g., `Timeout`).
    #[instrument(skip(self, operation))]
    pub async fn with_retry<R, Fut>(
        &self,
        operation: impl Fn() -> Fut,
        max_retries: u32,
    ) -> ServerStateResult<R>
    where
        Fut: std::future::Future<Output = ServerStateResult<R>>,
    {
        let mut attempts = 0;
        loop {
            attempts += 1;
            match timeout(self.config.request_timeout, operation()).await {
                Ok(Ok(result)) => return Ok(result),
                Ok(Err(e)) if e.is_retryable() && attempts <= max_retries => {
                    self.metrics.record_error(&e);
                    warn!(
                        "Attempt {attempts} failed with retryable error: {e}. Retrying in 1s..."
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Err(e)) => {
                    self.metrics.record_error(&e);
                    error!("Operation failed after {attempts} attempts: {e}");
                    return Err(e);
                }
                Err(_) => {
                    let timeout_error = ServerStateError::Timeout {
                        duration: self.config.request_timeout,
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            "Operation timed out",
                        )),
                    };
                    self.metrics.record_error(&timeout_error);
                    error!("Operation timed out after {attempts} attempts");
                    return Err(timeout_error);
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Unit Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;
    use proptest::prelude::*;

    #[traced_test]
    #[tokio::test]
    async fn test_document_operations() -> ServerStateResult<()> {
        let config = ModuleConfig::new()?;
        let metrics = Arc::new(MetricsCollector::new("server_state".to_string()));
        let state = ServerState::new(config, metrics);

        let uri = "test_uri".to_string();
        let content = "test_content".to_string();

        // Insert document
        state.update_document(uri.clone(), content.clone());
        assert_eq!(state.get_document(&uri).await?, content);

        // Delete document
        state.delete_document(&uri)?;
        assert!(matches!(
            state.get_document(&uri).await,
            Err(ServerStateError::DocumentNotFound(_))
        ));

        Ok(())
    }

    #[traced_test]
    #[tokio::test]
    async fn test_with_retry_success() -> ServerStateResult<()> {
        let config = ModuleConfig::new()?;
        let metrics = Arc::new(MetricsCollector::new("server_state".to_string()));
        let state = ServerState::new(config, metrics);

        // Operation that succeeds immediately.
        let result = state.with_retry(|| async { Ok(42) }, 3).await?;
        assert_eq!(result, 42);

        Ok(())
    }

    #[traced_test]
    #[tokio::test]
    async fn test_with_retry_timeout() -> ServerStateResult<()> {
        let config = Arc::new(ModuleConfig {
            request_timeout: Duration::from_millis(10),
            ..*ModuleConfig::new()?
        });
        let metrics = Arc::new(MetricsCollector::new("server_state".to_string()));
        let state = ServerState::new(config, metrics);

        // Operation that always times out
        let result = state
            .with_retry(
                || async {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    Ok::<_, ServerStateError>(42)
                },
                2,
            )
            .await;

        // Should fail with a timeout error
        assert!(matches!(result, Err(ServerStateError::Timeout { .. })));

        Ok(())
    }

    // Example property-based test for verifying doc insertion behavior
    proptest! {
        #[test]
        fn prop_insertion_increase_count(docs in prop::collection::vec(".*", 1..100)) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let config = ModuleConfig::new().unwrap();
                let metrics = Arc::new(MetricsCollector::new("server_state".to_string()));
                let state = ServerState::new(config, metrics);

                for (i, doc_content) in docs.iter().enumerate() {
                    let uri = format!("doc_{i}");
                    state.update_document(uri.clone(), doc_content.clone());
                }
                // The number of documents should match the length inserted
                assert_eq!(state.documents.len(), docs.len());
            });
        }
    }
}
