//! Semantic analysis for the Kymera programming language.

pub mod err;
pub mod analyzer;
pub mod types;

pub use err::{Error, Result};
pub use analyzer::Analyzer;
pub use types::*; 