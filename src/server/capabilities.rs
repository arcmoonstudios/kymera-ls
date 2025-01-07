// src/server/capabilities.rs
// ------------------------------------------------------------------------------->
//
//! Provides the LSP server capabilities configuration for the Kymera language server, merging
//! advanced Rust best practices and guidelines from the Rust Module Enhancement Guide v1.0.
//!
//! # Highlights
//! - **Incremental** text document syncing
//! - **CompletionProvider** with type-safe trigger characters (including Kymera-specific operators)
//! - Support for hover, definition, references, symbols, and more
//! - Semantic tokens with advanced concurrency-based initialization
//! - Idiomatic error handling using `thiserror`
//! - Type-state approach for configuration loading
//! - Adherence to recommended security, concurrency, and RAII resource management
//!
//! # Usage
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     match capabilities::initialize_capabilities("config/capabilities.json").await {
//!         Ok(server_caps) => println!("Capabilities loaded successfully: {:?}", server_caps),
//!         Err(e) => eprintln!("Failed to load capabilities: {}", e),
//!     }
//! }
//! ```
// ------------------------------------------------------------------------------->

use std::{sync::Arc, path::Path};
use thiserror::Error;
use tokio::time::{timeout, Duration};
use tower_lsp::lsp_types::{
    CompletionOptions, HoverProviderCapability, OneOf, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, SemanticTokensRegistrationOptions,
    TextDocumentRegistrationOptions, DocumentFilter, SemanticTokensOptions,
    SemanticTokensLegend, SignatureHelpOptions, WorkspaceServerCapabilities,
    WorkspaceFoldersServerCapabilities, TypeDefinitionProviderCapability,
    ImplementationProviderCapability, SemanticTokensServerCapabilities,
    WorkDoneProgressOptions, SemanticTokenType, StaticRegistrationOptions,
    CodeActionProviderCapability, FoldingRangeProviderCapability,
    CallHierarchyServerCapability, SelectionRangeProviderCapability,
    SemanticTokensFullOptions,
};
use serde::{Deserialize, Serialize};
use config::{Config, Environment, File};
use serde_with::serde_as;

/// Enum representing all possible trigger characters for the completion provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerCharacter {
    ImportDeclaration,
    ScopeResolution,
    StructureDefinition,
    EnumerationDefinition,
    ImplementationBlock,
    FunctionDefinition,
    SelfReference,
    SynchronousCall,
    AsynchronousCall,
    AsyncAwait,
    ErrorPropagation,
    MatchStatement,
    ForLoop,
    MutableDesignator,
    ImmutableDesignator,
    LineComment,
    DocumentationComment,
    AIassistedCodeGen,
    VERXDebugger,
    TypeHintI8,
    TypeHintI16,
    TypeHintI32,
    TypeHintI64,
    TypeHintI128,
    TypeHintISZE,
    TypeHintU8,
    TypeHintU16,
    TypeHintU32,
    TypeHintU64,
    TypeHintU128,
    TypeHintUSZE,
    TypeHintF32,
    TypeHintF64,
    TypeHintStrng,
    TypeHintOptn,
    TypeHintRes,
}

impl TriggerCharacter {
    /// Converts the enum variant to its corresponding string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ImportDeclaration => "des",
            Self::ScopeResolution => ":>",
            Self::StructureDefinition => "forma",
            Self::EnumerationDefinition => "enum",
            Self::ImplementationBlock => "imp",
            Self::FunctionDefinition => "fnc",
            Self::SelfReference => "soy",
            Self::SynchronousCall => "sn>",
            Self::AsynchronousCall => "xn>",
            Self::AsyncAwait => "w>?",
            Self::ErrorPropagation => "r?",
            Self::MatchStatement => "m>",
            Self::ForLoop => "4>",
            Self::MutableDesignator => "~",
            Self::ImmutableDesignator => "&",
            Self::LineComment => "|>",
            Self::DocumentationComment => "|D>",
            Self::AIassistedCodeGen => "|A>",
            Self::VERXDebugger => "<v?x>",
            Self::TypeHintI8 => "i8",
            Self::TypeHintI16 => "i16",
            Self::TypeHintI32 => "i32",
            Self::TypeHintI64 => "i64",
            Self::TypeHintI128 => "i128",
            Self::TypeHintISZE => "ISZE",
            Self::TypeHintU8 => "u8",
            Self::TypeHintU16 => "u16",
            Self::TypeHintU32 => "u32",
            Self::TypeHintU64 => "u64",
            Self::TypeHintU128 => "u128",
            Self::TypeHintUSZE => "USZE",
            Self::TypeHintF32 => "f32",
            Self::TypeHintF64 => "f64",
            Self::TypeHintStrng => "Strng",
            Self::TypeHintOptn => "Optn",
            Self::TypeHintRes => "Res",
        }
    }
}

/// Custom error type for capabilities configuration.
#[derive(Debug, Error)]
pub enum CapabilitiesError {
    #[error("Configuration loading failed: {0}")]
    ConfigLoadError(String),
    #[error("Operation timed out after {duration:?}")]
    Timeout {
        duration: Duration,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("Validation Error: {message}")]
    ValidationError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl CapabilitiesError {
    /// Checks if an error is retryable (e.g., timeouts).
    pub fn is_retryable(&self) -> bool {
        matches!(self, CapabilitiesError::Timeout { .. })
    }
}

/// Type alias for results with `CapabilitiesError`.
pub type CapabilitiesResult<T> = Result<T, CapabilitiesError>;

/// Provides dynamic configuration for LSP server capabilities.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesConfig {
    pub trigger_characters: Vec<TriggerCharacter>,
    pub language_id: String,
    pub file_scheme: String,
    /// Maximum number of retries when loading configuration files or resources.
    #[serde(default = "default_retry_limit")]
    pub max_retries: u32,
    /// Timeout duration for loading the configuration.
    #[serde_as(as = "serde_with::DurationSeconds<f64>")]
    #[serde(default = "default_timeout_duration")]
    pub load_timeout: Duration,
}

fn default_retry_limit() -> u32 {
    3
}

fn default_timeout_duration() -> Duration {
    Duration::from_secs(5)
}

/// Type-state approach: Uninitialized -> Initialized
pub struct ConfigLoader<State = Uninitialized> {
    config: Option<CapabilitiesConfig>,
    state: std::marker::PhantomData<State>,
}

pub struct Uninitialized;
pub struct Initialized;

impl ConfigLoader<Uninitialized> {
    pub fn new() -> Self {
        Self {
            config: None,
            state: std::marker::PhantomData,
        }
    }

    /// Loads the server configuration from JSON or environment.
    /// This uses a retry + timeout approach to handle transient failures.
    pub async fn load_config(
        self,
        config_path: &str,
    ) -> CapabilitiesResult<ConfigLoader<Initialized>> {
        let cfg = with_retry(
            || async {
                timeout(
                    default_timeout_duration(),
                    async {
                        let config = load_dynamic_config(config_path).await?;
                        validate_config(&config)?;
                        Ok(config)
                    },
                )
                .await
                .map_err(|e| CapabilitiesError::Timeout {
                    duration: default_timeout_duration(),
                    source: Box::new(e),
                })?
            },
            default_retry_limit(),
            default_timeout_duration(),
        )
        .await?;

        Ok(ConfigLoader {
            config: Some(cfg),
            state: std::marker::PhantomData,
        })
    }
}

impl ConfigLoader<Initialized> {
    pub fn into_config(self) -> CapabilitiesConfig {
        self.config.unwrap()
    }
}

/// Validate the loaded configuration.
fn validate_config(cfg: &CapabilitiesConfig) -> CapabilitiesResult<()> {
    if cfg.language_id.trim().is_empty() {
        return Err(CapabilitiesError::ValidationError {
            message: "language_id cannot be empty".into(),
            source: None,
        });
    }
    // More validations if needed...
    Ok(())
}

/// Loads server capabilities configuration from various sources (JSON, environment).
async fn load_dynamic_config(path: &str) -> CapabilitiesResult<CapabilitiesConfig> {
    if !Path::new(path).exists() {
        return Err(CapabilitiesError::ConfigLoadError(format!(
            "Config file not found: {}",
            path
        )));
    }

    let builder = Config::builder()
        .add_source(File::with_name(path))
        .add_source(Environment::with_prefix("KYMERA").separator("_"));
    let settings = builder.build().map_err(|e| {
        CapabilitiesError::ConfigLoadError(format!("Config build error: {}", e.to_string()))
    })?;

    let cfg: CapabilitiesConfig = settings
        .try_deserialize()
        .map_err(|e| CapabilitiesError::ConfigLoadError(e.to_string()))?;

    Ok(cfg)
}

/// Retrying mechanism with timeouts, based on the advanced error handling pattern.
async fn with_retry<T, F, Fut>(operation: F, max_retries: u32, _timeout_duration: Duration) -> CapabilitiesResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = CapabilitiesResult<T>>,
{
    let mut attempts = 0;
    loop {
        attempts += 1;
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) if e.is_retryable() && attempts < max_retries => {
                tokio::time::sleep(backoff_duration(attempts)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Simple backoff strategy based on attempt count.
fn backoff_duration(attempts: u32) -> Duration {
    Duration::from_millis(500 * attempts as u64)
}

/// Initializes the server capabilities using advanced concurrency and type-state loading.
///
/// # Errors
///
/// - `CapabilitiesError::ConfigLoadError` when the JSON config cannot be parsed
/// - `CapabilitiesError::ValidationError` if required fields are invalid
/// - `CapabilitiesError::Timeout` if loading times out
///
pub async fn initialize_capabilities(config_path: &str) -> CapabilitiesResult<ServerCapabilities> {
    // Load the configuration with type-state transitions
    let loader = ConfigLoader::new();
    let loader = loader.load_config(config_path).await?;
    let config = loader.into_config();

    // Build final LSP server capabilities
    Ok(build_server_capabilities(&config).await)
}

/// Builds the server capabilities based on the provided configuration.
pub async fn build_server_capabilities(config: &CapabilitiesConfig) -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(
                config
                    .trigger_characters
                    .iter()
                    .map(|c| c.as_str().to_string())
                    .collect(),
            ),
            ..Default::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        references_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        semantic_tokens_provider: Some(
            SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                SemanticTokensRegistrationOptions {
                    text_document_registration_options: TextDocumentRegistrationOptions {
                        document_selector: Some(vec![DocumentFilter {
                            language: Some(config.language_id.clone()),
                            scheme: Some(config.file_scheme.clone()),
                            pattern: None,
                        }]),
                    },
                    semantic_tokens_options: SemanticTokensOptions {
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::FUNCTION,
                                SemanticTokenType::METHOD,
                                SemanticTokenType::PROPERTY,
                                SemanticTokenType::VARIABLE,
                                SemanticTokenType::PARAMETER,
                                SemanticTokenType::TYPE,
                                SemanticTokenType::CLASS,
                                SemanticTokenType::ENUM,
                                SemanticTokenType::INTERFACE,
                                SemanticTokenType::STRUCT,
                                SemanticTokenType::TYPE_PARAMETER,
                                SemanticTokenType::ENUM_MEMBER,
                                SemanticTokenType::EVENT,
                                SemanticTokenType::NAMESPACE,
                                SemanticTokenType::COMMENT,
                                SemanticTokenType::STRING,
                                SemanticTokenType::NUMBER,
                                SemanticTokenType::REGEXP,
                                SemanticTokenType::OPERATOR,
                                SemanticTokenType::KEYWORD,
                            ],
                            token_modifiers: vec![],
                        },
                        range: Some(false),
                        full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
                    },
                    static_registration_options: StaticRegistrationOptions::default(),
                },
            ),
        ),
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        document_formatting_provider: Some(OneOf::Left(true)),
        document_range_formatting_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Left(true)),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        document_link_provider: None,
        color_provider: None,
        execute_command_provider: None,
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: None,
        }),
        call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        ..Default::default()
    }
}

/// Builds basic server capabilities with minimal functionality.
/// Used as a fallback when dynamic configuration fails.
pub fn build_basic_server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![":".to_string(), ">".to_string(), "|".to_string()]),
            ..Default::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        ..Default::default()
    }
}

// Example unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::write;
    use serde_json::json;

    #[tokio::test]
    async fn test_valid_config() {
        let test_config = json!({
            "trigger_characters": ["des", ":>", "fnc"],
            "language_id": "kymera",
            "file_scheme": "file",
            "max_retries": 2,
            "load_timeout": "3s"
        });
        let config_path = "test_capabilities.json";
        write(config_path, test_config.to_string()).await.unwrap();

        let result = initialize_capabilities(config_path).await;
        assert!(result.is_ok());
        let caps = result.unwrap();
        assert_eq!(
            caps.text_document_sync.unwrap(),
            TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)
        );
        assert!(caps.completion_provider.is_some());
        let completion = caps.completion_provider.unwrap();
        assert_eq!(completion.trigger_characters.unwrap().len(), 3);

        // Cleanup
        tokio::fs::remove_file(config_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_invalid_file_path() {
        let result = initialize_capabilities("non_existent.json").await;
        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                CapabilitiesError::ConfigLoadError(_) => (),
                _ => panic!("Expected CapabilitiesError::ConfigLoadError"),
            }
        }
    }
}

/// Simple demonstration of concurrency usage in other parts of the module.
/// Processes items concurrently with a limit on simultaneous tasks.
pub async fn process_items_concurrently<I, T, F, Fut>(
    items: I,
    concurrency_limit: usize,
    func: F,
) -> CapabilitiesResult<Vec<T>>
where
    I: IntoIterator,
    I::Item: Send + 'static,
    T: Send + 'static,
    F: Fn(I::Item) -> Fut + Send + Sync + Clone + 'static,
    Fut: std::future::Future<Output = CapabilitiesResult<T>> + Send + 'static,
{
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency_limit));
    let mut handles = Vec::new();

    for item in items {
        let sem = semaphore.clone();
        let f = func.clone();
        let handle = tokio::spawn(async move {
            let permit = sem.acquire().await.ok();
            let _permit = permit; // keep in scope
            f(item).await
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.map_err(|e| {
            CapabilitiesError::ConfigLoadError(format!("Task join error: {}", e))
        })??);
    }

    Ok(results)
}

/// Execute an operation with retry and timeout logic.
pub async fn execute<T, F, Fut>(operation: F, max_retries: u32, timeout_duration: Duration) -> CapabilitiesResult<T>
where
    T: Send + 'static,
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = CapabilitiesResult<T>> + Send + 'static,
{
    let mut attempts = 0;
    loop {
        attempts += 1;
        match timeout(timeout_duration, operation()).await {
            Ok(Ok(result)) => return Ok(result),
            Ok(Err(e)) if e.is_retryable() && attempts <= max_retries => {
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            Ok(Err(e)) => return Err(e),
            Err(e) => {
                if attempts <= max_retries {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                return Err(CapabilitiesError::Timeout {
                    duration: timeout_duration,
                    source: Box::new(e),
                });
            }
        }
    }
}
