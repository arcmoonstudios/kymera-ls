//! Error types for the Kymera analyzer.

use thiserror::Error as AnalyzerError;
use anyhow::Result as AnalyzerResult;

/// Custom error type for the analysis phase
#[derive(Debug, AnalyzerError)]
pub enum AnalysisError {
    /// Type-related errors
    #[error("Type error: {message}")]
    TypeError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Type parsing errors
    #[error("Type parsing error: {message}")]
    TypeParseError {
        message: String,
        type_str: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Type validation errors
    #[error("Type validation error: {message}")]
    TypeValidationError {
        message: String,
        type_name: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Type parameter errors
    #[error("Type parameter error: {message}")]
    TypeParameterError {
        message: String,
        param_name: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Symbol-related errors
    #[error("Symbol error: {message}")]
    SymbolError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Scope-related errors
    #[error("Scope error: {message}")]
    ScopeError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Semantic analysis errors
    #[error("Semantic error: {message}")]
    SemanticError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Parser errors
    #[error("Parser error: {0}")]
    Parser(#[from] kymera_parser::err::ParserError),

    /// Core errors
    #[error("Core error: {0}")]
    Core(#[from] kymera_core::err::CoreError),

    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl AnalysisError {
    /// Creates a new type error
    pub fn type_error<S: Into<String>>(message: S) -> Self {
        Self::TypeError {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new type parsing error
    pub fn type_parse_error<S: Into<String>>(message: S, type_str: S) -> Self {
        Self::TypeParseError {
            message: message.into(),
            type_str: type_str.into(),
            source: None,
        }
    }

    /// Creates a new type validation error
    pub fn type_validation_error<S: Into<String>>(message: S, type_name: S) -> Self {
        Self::TypeValidationError {
            message: message.into(),
            type_name: type_name.into(),
            source: None,
        }
    }

    /// Creates a new type parameter error
    pub fn type_parameter_error<S: Into<String>>(message: S, param_name: S) -> Self {
        Self::TypeParameterError {
            message: message.into(),
            param_name: param_name.into(),
            source: None,
        }
    }

    /// Creates a new symbol error
    pub fn symbol_error<S: Into<String>>(message: S) -> Self {
        Self::SymbolError {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new scope error
    pub fn scope_error<S: Into<String>>(message: S) -> Self {
        Self::ScopeError {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new semantic error
    pub fn semantic_error<S: Into<String>>(message: S) -> Self {
        Self::SemanticError {
            message: message.into(),
            source: None,
        }
    }

    /// Adds a source error to an existing error
    pub fn with_source<E>(mut self, err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        match &mut self {
            Self::TypeError { source, .. } |
            Self::TypeParseError { source, .. } |
            Self::TypeValidationError { source, .. } |
            Self::TypeParameterError { source, .. } |
            Self::SymbolError { source, .. } |
            Self::ScopeError { source, .. } |
            Self::SemanticError { source, .. } => {
                *source = Some(Box::new(err));
            }
            _ => {}
        }
        self
    }
}

/// Result type alias for the analysis phase
pub type Result<T> = AnalyzerResult<T>; 