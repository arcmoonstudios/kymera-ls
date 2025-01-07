//! Neural-symbolic AI core for the Kymera programming language.
//!
//! This crate provides a multi-layered system for quantum-enhanced
//! debugging, meta-learning, and neural-state processing.

pub mod err;
pub mod lsnsn;
pub mod mtalr;
pub mod verx;

pub use err::{CortexError, Result};
pub use lsnsn::{LSNsN, LSNsNConfig, NeuralState, StateType};
pub use mtalr::{MTALR, MTALRError};
pub use verx::{VerxConfig, VerxSystem as Verx};
