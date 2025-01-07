//! Error types for the Kymera cortex.
//! 
//! This module provides specialized error types for neural network operations,
//! quantum computing, and AI debugging functionality.

use thiserror::Error;
use crate::{
    lsnsn::{quantum, learning, reservoir},
    mtalr::MTALRError,
};

/// Core error type for the Kymera cortex
#[derive(Debug, Error)]
pub enum CortexError {
    #[error("VERX error: {0}")]
    Verx(#[from] VerxError),

    #[error("MTALR error: {0}")]
    Mtalr(#[from] MtalrError),

    #[error("LSNSN error: {0}")]
    Lsnsn(#[from] LsnsnError),

    #[error("Tape error: {0}")]
    Tape(TapeError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// VERX AI debugger error type
#[derive(Debug, Error)]
pub enum VerxError {
    #[error("Context error: {0}")]
    Context(#[from] ContextError),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Pattern matching error: {0}")]
    PatternMatching(String),

    #[error("Debug trace error: {0}")]
    Trace(String),

    #[error("Breakpoint error: {0}")]
    Breakpoint(String),
}

/// Context management error type
#[derive(Debug, Error)]
pub enum ContextError {
    #[error("Context initialization error: {0}")]
    Initialization(String),

    #[error("Context update error: {0}")]
    Update(String),

    #[error("Context validation error: {0}")]
    Validation(String),

    #[error("Context persistence error: {0}")]
    Persistence(String),
}

/// MTALR (Meta-Turing Adaptive Learning & Reasoning) error type
#[derive(Debug, Error)]
pub enum MtalrError {
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    #[error("Learning error: {0}")]
    Learning(#[from] LearningError),

    #[error("Tape error: {0}")]
    Tape(#[from] TapeError),

    #[error("Reasoning error: {0}")]
    Reasoning(String),

    #[error("Meta-learning error: {0}")]
    MetaLearning(String),
}

/// Core processing error type
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Processing error: {0}")]
    Processing(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Computation error: {0}")]
    Computation(String),
}

/// Learning system error type
#[derive(Debug, Error)]
pub enum LearningError {
    #[error("Training error: {0}")]
    Training(String),

    #[error("Optimization error: {0}")]
    Optimization(String),

    #[error("Model validation error: {0}")]
    ModelValidation(String),

    #[error("Dataset error: {0}")]
    Dataset(String),
}

/// Tape management error type
#[derive(Debug, Error, PartialEq, Clone)]
pub enum TapeError {
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),

    #[error("Out of bounds access: {0}")]
    OutOfBounds(String),

    #[error("Quantum state error: {0}")]
    QuantumError(String),

    #[error("Time error: {0}")]
    TimeError(String),

    #[error("Read error: {0}")]
    Read(String),

    #[error("Write error: {0}")]
    Write(String),

    #[error("Seek error: {0}")]
    Seek(String),

    #[error("Bounds error: {0}")]
    Bounds(String),
}

/// LSNSN (Liquid State NeuroSymbolic Network) error type
#[derive(Debug, Error)]
pub enum LsnsnError {
    #[error("Quantum error: {0}")]
    Quantum(#[from] QuantumError),

    #[error("Reservoir error: {0}")]
    Reservoir(#[from] ReservoirError),

    #[error("Learning error: {0}")]
    Learning(String),

    #[error("Conversion error: {0}")]
    Conversion(String),
}

/// Quantum computation error type
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
}

/// Reservoir computing error type
#[derive(Debug, Error)]
pub enum ReservoirError {
    #[error("Initialization error: {0}")]
    Initialization(String),

    #[error("State update error: {0}")]
    Update(String),

    #[error("Readout error: {0}")]
    Readout(String),
}

/// Result type alias for cortex operations
pub type Result<T> = std::result::Result<T, CortexError>;

// Also expose specific Result types for each subsystem
pub type VerxResult<T> = Result<T>;
pub type MtalrResult<T> = Result<T>;
pub type LsnsnResult<T> = Result<T>;
pub type TapeResult<T> = std::result::Result<T, TapeError>;

/// Helper trait for adding context to errors
pub trait Context<T, E> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> Context<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| CortexError::Internal(format!("{}: {}", context, e)))
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| CortexError::Internal(format!("{}: {}", f(), e)))
    }
}

// Implement conversions between error types
impl From<ContextError> for CortexError {
    fn from(err: ContextError) -> Self {
        Self::Verx(VerxError::Context(err))
    }
}

impl From<CoreError> for CortexError {
    fn from(err: CoreError) -> Self {
        Self::Mtalr(MtalrError::Core(err))
    }
}

impl From<LearningError> for CortexError {
    fn from(err: LearningError) -> Self {
        Self::Mtalr(MtalrError::Learning(err))
    }
}

impl From<TapeError> for CortexError {
    fn from(err: TapeError) -> Self {
        Self::Tape(err)
    }
}

impl From<QuantumError> for CortexError {
    fn from(err: QuantumError) -> Self {
        Self::Lsnsn(LsnsnError::Quantum(err))
    }
}

impl From<ReservoirError> for CortexError {
    fn from(err: ReservoirError) -> Self {
        Self::Lsnsn(LsnsnError::Reservoir(err))
    }
}

// Add From implementations for reservoir errors
impl From<reservoir::ReservoirError> for CortexError {
    fn from(err: reservoir::ReservoirError) -> Self {
        CortexError::Internal(format!("Reservoir error: {}", err))
    }
}

// Add From implementations for quantum errors
impl From<quantum::QuantumError> for CortexError {
    fn from(err: quantum::QuantumError) -> Self {
        CortexError::Internal(format!("Quantum error: {}", err))
    }
}

// Add From implementations for learning errors
impl From<learning::LearningError> for CortexError {
    fn from(err: learning::LearningError) -> Self {
        CortexError::Internal(format!("Learning error: {}", err))
    }
}

// Add From implementation for converting Error to MTALRError
impl From<CortexError> for MTALRError {
    fn from(err: CortexError) -> Self {
        match err {
            CortexError::Mtalr(e) => MTALRError::Other(e.to_string()),
            CortexError::Internal(msg) => MTALRError::Other(msg),
            _ => MTALRError::Other(err.to_string())
        }
    }
}
