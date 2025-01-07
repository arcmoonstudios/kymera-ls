use std::{
    sync::Arc,
    time::SystemTime,
};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use num_complex::Complex64;

use crate::err::Result;

pub mod quantum;
pub mod learning;
pub mod reservoir;

use self::{
    quantum::{QuantumInterface, QuantumConfig},
    learning::{LearningSystem, LearningConfig},
    reservoir::{ReservoirSystem, ReservoirConfig},
};

/// Neural input type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralInput {
    pub values: Vec<f64>,
    pub timestamp: SystemTime,
    pub metadata: StateMetadata,
}

/// Neural target type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralTarget {
    pub values: Vec<f64>,
    pub timestamp: SystemTime,
    pub metadata: StateMetadata,
}

/// Neural state type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralState {
    pub values: Vec<f64>,
    pub timestamp: SystemTime,
    pub metadata: StateMetadata,
}

impl Default for NeuralState {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            timestamp: SystemTime::now(),
            metadata: StateMetadata::default(),
        }
    }
}

/// State metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMetadata {
    pub state_type: StateType,
    pub confidence: f64,
    pub timestamp: SystemTime,
}

impl Default for StateMetadata {
    fn default() -> Self {
        Self {
            state_type: StateType::default(),
            confidence: 0.0,
            timestamp: SystemTime::now(),
        }
    }
}

/// State type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StateType {
    #[default]
    Input,
    Hidden,
    Output,
}

/// LSNsN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSNsNConfig {
    pub quantum: QuantumConfig,
    pub learning: LearningConfig,
    pub reservoir: ReservoirConfig,
}

/// Learning output type
#[derive(Debug, Clone)]
pub struct LearningOutput {
    pub loss: f64,
    pub gradients: Vec<Complex64>,
}

/// Complex neural target type
#[derive(Debug, Clone)]
pub struct ComplexNeuralTarget(pub Vec<Complex64>);

/// LSNsN implementation
#[derive(Debug)]
pub struct LSNsN {
    #[allow(dead_code)]
    config: LSNsNConfig,
    quantum_interface: Arc<RwLock<QuantumInterface>>,
    learning_system: Arc<RwLock<LearningSystem>>,
    reservoir: Arc<RwLock<ReservoirSystem>>,
    _state: Arc<RwLock<NeuralState>>,
}

impl LSNsN {
    pub async fn new(config: LSNsNConfig) -> Result<Self> {
        let quantum_interface = Arc::new(RwLock::new(QuantumInterface::new(config.quantum.clone())));
        let learning_system = Arc::new(RwLock::new(LearningSystem::new(config.learning.clone())));
        let reservoir = Arc::new(RwLock::new(ReservoirSystem::new(config.reservoir.clone())?));

        // Initialize quantum interface
        {
            let mut interface = quantum_interface.write().await;
            interface.initialize().await?;
        }

        Ok(Self {
            config,
            quantum_interface,
            learning_system,
            reservoir,
            _state: Arc::new(RwLock::new(NeuralState::default())),
        })
    }

    pub async fn reset(&self) -> Result<()> {
        {
            let mut interface = self.quantum_interface.write().await;
            interface.initialize().await?;
        }

        {
            let mut learning = self.learning_system.write().await;
            learning.prepare_learning().await?;
        }

        {
            let mut reservoir = self.reservoir.write().await;
            reservoir.prepare_processing().await?;
        }

        Ok(())
    }

    pub async fn process(&self, input: NeuralInput) -> Result<NeuralState> {
        let quantum_update = {
            let interface = self.quantum_interface.read().await;
            interface.process_input(&input).await?
        };

        let reservoir_state = {
            let mut reservoir = self.reservoir.write().await;
            reservoir.process_quantum_state(&quantum_update.target_state).await?
        };

        Ok(NeuralState {
            values: reservoir_state.values.iter().map(|c| c.norm()).collect(),
            timestamp: SystemTime::now(),
            metadata: StateMetadata {
                state_type: StateType::Output,
                confidence: reservoir_state.confidence,
                timestamp: SystemTime::now(),
            },
        })
    }

    pub async fn train(&self, target: NeuralTarget) -> Result<()> {
        let input = NeuralInput {
            values: target.values.clone(),
            timestamp: target.timestamp,
            metadata: target.metadata,
        };

        let quantum_update = {
            let interface = self.quantum_interface.read().await;
            interface.process_input(&input).await?
        };

        let target_array = ndarray::Array1::from_vec(
            quantum_update.target_state.amplitudes.clone()
        );

        let input_array = ndarray::Array1::from_vec(
            input.values.into_iter()
                .map(|v| Complex64::new(v, 0.0))
                .collect()
        );

        {
            let mut learning = self.learning_system.write().await;
            learning.train_step(&input_array, &target_array).await?;
        }

        Ok(())
    }
}

impl NeuralState {
    pub fn generate_insights(&self) -> Vec<crate::verx::Insight> {
        self.values.iter()
            .enumerate()
            .map(|(i, &value)| crate::verx::Insight {
                quantum_probability: value,
                explanation: format!("Neural insight {}: {:.3}", i, value),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> LSNsNConfig {
        LSNsNConfig {
            quantum: QuantumConfig::default(),
            learning: LearningConfig::default(),
            reservoir: ReservoirConfig::default(),
        }
    }

    fn create_test_input() -> NeuralInput {
        NeuralInput {
            values: vec![1.0, 0.0, -1.0],
            timestamp: SystemTime::now(),
            metadata: StateMetadata::default(),
        }
    }

    fn create_test_target() -> NeuralTarget {
        NeuralTarget {
            values: vec![0.0, 1.0, 0.0],
            timestamp: SystemTime::now(),
            metadata: StateMetadata::default(),
        }
    }

    #[tokio::test]
    async fn test_initialization() -> Result<()> {
        let lsnsn = LSNsN::new(create_test_config()).await?;
        assert!(Arc::strong_count(&lsnsn.quantum_interface) == 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_processing() -> Result<()> {
        let lsnsn = LSNsN::new(create_test_config()).await?;
        let input = create_test_input();
        let state = lsnsn.process(input).await?;
        assert!(!state.values.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_training() -> Result<()> {
        let lsnsn = LSNsN::new(create_test_config()).await?;
        let target = create_test_target();
        lsnsn.train(target).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_reset() -> Result<()> {
        let lsnsn = LSNsN::new(create_test_config()).await?;
        lsnsn.reset().await?;
        Ok(())
    }
}