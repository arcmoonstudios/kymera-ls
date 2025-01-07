use std::{
    sync::Arc,
    time::{SystemTime, Instant},
};
use tokio::sync::RwLock;
use anyhow::Context as _;
use thiserror::Error;
use tracing::{info, debug, instrument};
use uuid::Uuid;

pub mod quantum;
pub mod context;

use quantum::{QuantumConfig, QuantumState, PatternState, QuantumError};

use crate::{
    lsnsn::{LSNsN, LSNsNConfig, NeuralInput, StateMetadata, quantum as lsnsn_quantum},
    mtalr::{self, MTALR, MTALRConfig},
    verx::{MetaAnalysis, Insight},
};

/// Errors that can occur during debugging operations
#[derive(Error, Debug)]
pub enum DebuggerError {
    #[error("Neural engine error: {0}")]
    NeuralEngine(String),

    #[error("Meta engine error: {0}")]
    MetaEngine(String),

    #[error("Quantum state error: {0}")]
    QuantumState(#[from] QuantumError),

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type for debugger operations
pub type Result<T> = std::result::Result<T, DebuggerError>;

/// Analysis result from the async debugger
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub patterns: Vec<PatternState>,
    pub insights: Vec<Insight>,
    pub confidence: f64,
    pub quantum_state: QuantumState,
    pub timestamp: SystemTime,
}

/// Core trait for asynchronous debugging operations
#[async_trait::async_trait]
pub trait Debugger: Send + Sync + 'static {
    async fn new() -> Result<Self> where Self: Sized;
    async fn analyze(&self, code: &str) -> Result<AnalysisResult>;
}

/// VERX debugger implementation with quantum capabilities
#[derive(Debug)]
pub struct VERXDebugger {
    neural_engine: Arc<RwLock<LSNsN>>,
    meta_engine: Arc<RwLock<MTALR>>,
    config: QuantumConfig,
}

#[async_trait::async_trait]
impl Debugger for VERXDebugger {
    #[instrument(name = "verx_debugger_new", skip_all, err(Debug))]
    async fn new() -> Result<Self> {
        info!("Initializing VERX AI Debugger");

        let config = QuantumConfig::default();
        debug!(?config, "Using quantum configuration");

        let lsnsn_config = LSNsNConfig {
            quantum: lsnsn_quantum::QuantumConfig {
                num_qubits: config.num_qubits,
                circuit_depth: config.circuit_depth,
                error_correction: config.error_correction,
                memory_size: 1024,
                entanglement_params: Default::default(),
            },
            learning: Default::default(),
            reservoir: Default::default(),
        };

        let lsnsn = LSNsN::new(lsnsn_config).await
            .context("Failed to initialize LSNsN")?;
        let neural_engine = Arc::new(RwLock::new(lsnsn));

        let mtalr = MTALR::new(MTALRConfig::default())
            .context("Failed to initialize MTALR")?;
        let meta_engine = Arc::new(RwLock::new(mtalr));

        Ok(Self {
            neural_engine,
            meta_engine,
            config,
        })
    }

    #[instrument(skip(self, code), fields(code_len = code.len()), err(Debug))]
    async fn analyze(&self, code: &str) -> Result<AnalysisResult> {
        debug!("Starting code analysis");

        let quantum_state = QuantumState::new(self.config.num_qubits as f64);
        let state_vec = quantum_state.process_code(code)
            .map_err(DebuggerError::QuantumState)?;

        let pattern_state = PatternState {
            id: Uuid::new_v4(),
            state: state_vec,
            pattern_type: "quantum".to_string(),
            confidence: 1.0,
            created_at: SystemTime::now(),
        };

        let neural_input = NeuralInput {
            values: vec![pattern_state.confidence],
            timestamp: SystemTime::now(),
            metadata: StateMetadata::default(),
        };
        
        let neural_analysis = {
            let engine = self.neural_engine.read().await;
            engine.process(neural_input).await
                .map_err(|e| DebuggerError::NeuralEngine(e.to_string()))?
        };

        let meta_analysis = {
            let engine = self.meta_engine.read().await;
            engine.process_reasoning(&mtalr::MetaInput {
                data: neural_analysis.values.iter().flat_map(|x| x.to_le_bytes()).collect(),
                timestamp: Instant::now(),
            }).await
                .map_err(|e| DebuggerError::MetaEngine(e.to_string()))?
        };

        self.generate_insights(pattern_state, meta_analysis)
    }
}

impl VERXDebugger {
    fn generate_insights(
        &self,
        pattern_state: PatternState,
        meta_analysis: MetaAnalysis,
    ) -> Result<AnalysisResult> {
        debug!("Generating insights from analysis");

        let insights = meta_analysis.generate_insights()
            .map_err(|e| DebuggerError::Analysis(e.to_string()))?;
        let confidence = calculate_confidence(&insights);

        let quantum_state = QuantumState::new(self.config.num_qubits as f64);
        quantum_state.apply_meta_analysis(&meta_analysis)
            .map_err(DebuggerError::QuantumState)?;

        Ok(AnalysisResult {
            patterns: vec![pattern_state],
            insights,
            confidence,
            quantum_state,
            timestamp: SystemTime::now(),
        })
    }
}

/// Calculates confidence score from quantum insights
#[inline]
fn calculate_confidence(insights: &[Insight]) -> f64 {
    if insights.is_empty() {
        return 0.0;
    }
    
    insights.iter()
        .map(|ins| ins.quantum_probability)
        .sum::<f64>()
        .div_euclid(insights.len() as f64)
        .clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_debugger_initialization() {
        let debugger = VERXDebugger::new().await;
        assert!(debugger.is_ok());
    }

    #[test]
    fn test_confidence_calculation() {
        let insights = vec![
            Insight {
                quantum_probability: 0.8,
                ..Default::default()
            },
            Insight {
                quantum_probability: 0.6,
                ..Default::default()
            },
        ];
        
        let confidence = calculate_confidence(&insights);
        assert!((confidence - 0.7).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_empty_code_analysis() -> Result<()> {
        let debugger = VERXDebugger::new().await?;
        let result = debugger.analyze("").await;
        assert!(result.is_ok());
        Ok(())
    }
}