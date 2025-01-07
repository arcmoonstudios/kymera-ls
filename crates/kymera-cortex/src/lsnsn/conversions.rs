//! Conversion implementations for neural types

use std::time::SystemTime;
use std::time::Instant;
use crate::{
    verx::debugger::quantum::{PatternState, PatternCorrelation},
    lsnsn::{NeuralInput, StateMetadata, StateType},
    mtalr::MetaInput,
    lsnsn::NeuralState,
};

impl NeuralInput {
    /// Create NeuralInput from quantum patterns
    pub fn from_quantum_patterns(patterns: &[PatternState]) -> Self {
        let values = patterns.iter()
            .flat_map(|p| p.state.clone())
            .collect();

        Self {
            values,
            timestamp: SystemTime::now(),
            metadata: StateMetadata {
                state_type: StateType::Input,
                confidence: 1.0,
                timestamp: SystemTime::now(),
            },
        }
    }
}

impl From<Vec<PatternState>> for NeuralInput {
    fn from(patterns: Vec<PatternState>) -> Self {
        Self::from_quantum_patterns(&patterns)
    }
}

impl From<Vec<PatternCorrelation>> for NeuralInput {
    fn from(correlations: Vec<PatternCorrelation>) -> Self {
        let values = correlations.iter()
            .map(|c| c.quantum_prob)
            .collect();

        Self {
            values,
            timestamp: SystemTime::now(),
            metadata: StateMetadata {
                state_type: StateType::Input,
                confidence: 1.0,
                timestamp: SystemTime::now(),
            },
        }
    }
}

impl From<NeuralState> for MetaInput {
    fn from(state: NeuralState) -> Self {
        // Serialize the neural state values into bytes
        let data = state.values.iter()
            .flat_map(|&v| v.to_le_bytes().to_vec())
            .collect();

        MetaInput {
            data,
            timestamp: Instant::now(),
        }
    }
} 