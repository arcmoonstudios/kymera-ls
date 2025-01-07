//! Core types and utilities for the Kymera programming language.
//! This crate provides shared functionality used across the Kymera ecosystem.

pub mod err;
pub mod types;
pub mod utils;

pub use err::{Error, Result};

/// Re-export common traits and types
pub mod prelude {
    pub use crate::err::{Error, Result};
    pub use crate::utils::*;
}
