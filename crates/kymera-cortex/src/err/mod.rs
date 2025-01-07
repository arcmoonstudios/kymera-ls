//! Error types for the Kymera cortex.
//! 
//! This module provides a comprehensive error handling system for the Kymera cortex,
//! using thiserror for defining error types and anyhow for error context and propagation.
//! 
//! # Error Hierarchy
//! - CortexError: Top-level error type that encompasses all possible errors
//!   - NeuralError: Neural network specific errors
//!   - QuantumError: Quantum computation errors
//!   - StateError: State management errors
//!   - SystemError: System-level errors
//!   - VerxError: AI debugger errors
//!   - MTALRError: Meta-Turing Adaptive Learning errors
//!   - CoreError: Core processing errors
//!   - LearningError: Learning system errors
//!   - TapeError: Tape management errors
//!   - AdaptiveError: Adaptive reasoning errors
//!   - ContextError: Context management errors

pub use thiserror::Error;
pub use anyhow::{Context, Result, anyhow, bail, ensure};
use std::fmt::Display;

/// Neural-specific error type
#[derive(Debug, Error)]
pub enum NeuralError {
    #[error("Initialization error: {0}")]
    Initialization(String),

    #[error("Training error: {0}")]
    Training(String),

    #[error("Inference error: {0}")]
    Inference(String),

    #[error("Shape mismatch: {0}")]
    ShapeMismatch(String),

    #[error("Device error: {0}")]
    Device(String),

    #[error("Gradient computation error: {0}")]
    Gradient(String),

    #[error("Backpropagation error: {0}")]
    Backpropagation(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Quantum-specific error type
#[derive(Debug, Error)]
pub enum QuantumError {
    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error("State preparation error: {0}")]
    StatePreparation(String),

    #[error("Measurement error: {0}")]
    Measurement(String),

    #[error("Decoherence error: {0}")]
    Decoherence(String),

    #[error("Quantum gate error: {0}")]
    Gate(String),

    #[error("Entanglement error: {0}")]
    Entanglement(String),

    #[error("Quantum memory error: {0}")]
    Memory(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// State management error type
#[derive(Debug, Error)]
pub enum StateError {
    #[error("State initialization error: {0}")]
    Initialization(String),

    #[error("State update error: {0}")]
    Update(String),

    #[error("State compression error: {0}")]
    Compression(String),

    #[error("History tracking error: {0}")]
    History(String),

    #[error("State validation error: {0}")]
    Validation(String),

    #[error("State persistence error: {0}")]
    Persistence(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// System-level error type
#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Resource error: {0}")]
    Resource(String),

    #[error("Concurrency error: {0}")]
    Concurrency(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Memory allocation error: {0}")]
    MemoryAllocation(String),

    #[error("Thread error: {0}")]
    Thread(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// VERX AI debugger error type
#[derive(Debug, Error)]
pub enum VerxError {
    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Pattern matching error: {0}")]
    PatternMatching(String),

    #[error("Context error: {0}")]
    Context(String),

    #[error("Quantum debugging error")]
    QuantumDebugging(#[from] QuantumError),

    #[error("Debug trace error: {0}")]
    Trace(String),

    #[error("Breakpoint error: {0}")]
    Breakpoint(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Meta-Turing Adaptive Learned Reasoning error type
#[derive(Debug, Error)]
pub enum MTALRError {
    #[error("Core error")]
    Core(#[from] CoreError),

    #[error("Learning error")]
    Learning(#[from] LearningError),

    #[error("Tape error")]
    Tape(#[from] TapeError),

    #[error("Adaptive error")]
    Adaptive(#[from] AdaptiveError),

    #[error("Reasoning error: {0}")]
    Reasoning(String),

    #[error("Meta-learning error: {0}")]
    MetaLearning(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Core processing error type
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Processing error: {0}")]
    Processing(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("State error")]
    State(#[from] StateError),

    #[error("Computation error: {0}")]
    Computation(String),

    #[error("Resource allocation error: {0}")]
    ResourceAllocation(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Learning system error type
#[derive(Debug, Error)]
pub enum LearningError {
    #[error("Training error: {0}")]
    Training(String),

    #[error("Optimization error: {0}")]
    Optimization(String),

    #[error("Neural error")]
    Neural(#[from] NeuralError),

    #[error("Model validation error: {0}")]
    ModelValidation(String),

    #[error("Dataset error: {0}")]
    Dataset(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Tape management error type
#[derive(Debug, Error)]
pub enum TapeError {
    #[error("Read error: {0}")]
    Read(String),

    #[error("Write error: {0}")]
    Write(String),

    #[error("Seek error: {0}")]
    Seek(String),

    #[error("Bounds error: {0}")]
    Bounds(String),

    #[error("Quantum state error: {0}")]
    QuantumState(String),

    #[error("Time error: {0}")]
    Time(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Adaptive reasoning error type
#[derive(Debug, Error)]
pub enum AdaptiveError {
    #[error("Pattern error: {0}")]
    Pattern(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Reasoning error: {0}")]
    Reasoning(String),

    #[error("Adaptation error: {0}")]
    Adaptation(String),

    #[error("Strategy error: {0}")]
    Strategy(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Context management error type
#[derive(Debug, Error)]
pub enum ContextError {
    #[error("Context initialization error: {0}")]
    Initialization(String),

    #[error("Context update error: {0}")]
    Update(String),

    #[error("Context search error: {0}")]
    Search(String),

    #[error("Context validation error: {0}")]
    Validation(String),

    #[error("Context persistence error: {0}")]
    Persistence(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Main error type for the Kymera cortex
#[derive(Debug, Error)]
pub enum CortexError {
    #[error("Neural error")]
    Neural(#[from] NeuralError),

    #[error("Quantum error")]
    Quantum(#[from] QuantumError),

    #[error("State error")]
    State(#[from] StateError),

    #[error("System error")]
    System(#[from] SystemError),

    #[error("VERX error")]
    Verx(#[from] VerxError),

    #[error("MTALR error")]
    MTALR(#[from] MTALRError),

    #[error("Core error")]
    Core(#[from] CoreError),

    #[error("Learning error")]
    Learning(#[from] LearningError),

    #[error("Tape error")]
    Tape(#[from] TapeError),

    #[error("Adaptive error")]
    Adaptive(#[from] AdaptiveError),

    #[error("Context error")]
    Context(#[from] ContextError),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Helper trait for adding context to errors
pub trait ErrorExt<T> {
    /// Adds context to an error using anyhow's Context trait
    fn with_ctx<C, F>(self, context: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ErrorExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_ctx<C, F>(self, context: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| anyhow::Error::new(e).context(context()))
    }
}

// Helper functions for common error patterns
pub mod prelude {
    pub use super::*;

    /// Creates a new error with context
    pub fn with_context<T, C>(result: Result<T>, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        result.context(context)
    }

    /// Ensures a condition is true, otherwise returns an error
    pub fn ensure_with<T>(condition: bool, error: T) -> Result<()>
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        if condition {
            Ok(())
        } else {
            Err(anyhow::Error::new(error))
        }
    }
}
