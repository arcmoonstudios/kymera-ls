//! Analysis module for the Kymera programming language.

pub mod analyzer;
pub mod err;
pub mod symbols;
pub mod types;

pub use analyzer::Analyzer;
pub use err::{AnalysisError, Result};
pub use symbols::{AnalysisSymbol, AnalysisTable, SymbolKind, Visibility};
pub use types::{Type, TypeChecker, FunctionType, StructType, EnumType};

// Re-export anyhow for users of this crate
pub use anyhow; 