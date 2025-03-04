// crates/kymera-reactor/src/lib.rs

//! Reactive compiler for the Kymera programming language.

pub mod err;
pub mod traits;
pub mod types;

pub use err::ReactorError;
pub use traits::*;
pub use types::{ReactorResult, ModuleResult};

