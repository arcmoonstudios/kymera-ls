//! Language server implementation for the Kymera programming language.

pub mod err;
pub mod server;

pub use err::{Error, Result};
pub use tower_lsp::Server;
