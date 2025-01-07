// src/lsnsn/learning.rs

use std::{
    sync::Arc,
    time::SystemTime,
};
use ndarray::{Array1, Array2};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info, instrument, warn};

use super::{
    quantum::{QuantumInterface, QuantumUpdate, QuantumConfig},
    LearningOutput,
    ComplexNeuralTarget,
    NeuralInput,
    StateMetadata,
};

/// Learning system errors
#[derive(Error, Debug)]
pub enum LearningError {
    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("Training error: {0}")]
    TrainingError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("Quantum error: {0}")]
    QuantumError(String),
}

/// Learning system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Hidden state dimension
    pub hidden_dim: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Momentum coefficient
    pub momentum: f64,
    /// L2 regularization strength
    pub l2_reg: f64,
    /// Quantum learning enabled
    pub enable_quantum: bool,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            hidden_dim: 64,
            learning_rate: 0.01,
            momentum: 0.9,
            l2_reg: 0.0001,
            enable_quantum: true,
        }
    }
}

/// Learning system state
#[derive(Debug, Clone)]
pub struct LearningState {
    /// Current weights
    weights: Array2<Complex64>,
    /// Weight velocities (for momentum)
    velocities: Array2<Complex64>,
    /// Learning statistics
    stats: LearningStats,
}

/// Learning statistics
#[derive(Debug, Clone, Default)]
pub struct LearningStats {
    /// Total training steps
    total_steps: usize,
    /// Current loss value
    current_loss: f64,
    /// Moving average of loss
    avg_loss: f64,
    /// Best loss achieved
    best_loss: f64,
    /// Steps since last improvement
    steps_no_improve: usize,
}

/// Learning system implementation
#[derive(Debug)]
pub struct LearningSystem {
    /// Configuration
    config: LearningConfig,
    /// Current state
    state: LearningState,
    /// Quantum interface
    quantum: Arc<QuantumInterface>,
}

impl LearningSystem {
    /// Create new learning system
    pub fn new(config: LearningConfig) -> Self {
        let weights = Array2::zeros((config.hidden_dim, config.hidden_dim));
        let velocities = Array2::zeros((config.hidden_dim, config.hidden_dim));
        
        let state = LearningState {
            weights,
            velocities,
            stats: LearningStats::default(),
        };

        Self {
            config,
            state,
            quantum: Arc::new(QuantumInterface::new(QuantumConfig::default())),
        }
    }

    /// Initialize learning system
    #[instrument(skip(self))]
    pub fn initialize(&mut self) -> Result<(), LearningError> {
        info!("Initializing learning system");

        // Initialize weights with small random values
        self.state.weights.mapv_inplace(|_| {
            Complex64::new(
                rand::random::<f64>() * 0.1 - 0.05,
                rand::random::<f64>() * 0.1 - 0.05
            )
        });

        // Reset velocities
        self.state.velocities.fill(Complex64::new(0.0, 0.0));

        // Reset statistics
        self.state.stats = LearningStats {
            best_loss: f64::INFINITY,
            ..Default::default()
        };

        debug!("Learning system initialized");
        Ok(())
    }

    /// Prepare for learning phase
    #[instrument(skip(self))]
    pub async fn prepare_learning(&mut self) -> Result<(), LearningError> {
        info!("Preparing learning system");

        // Reset velocities
        self.state.velocities.fill(Complex64::new(0.0, 0.0));

        // Reset statistics
        self.state.stats = LearningStats {
            best_loss: f64::INFINITY,
            ..Default::default()
        };

        Ok(())
    }

    /// Process quantum update
    #[instrument(skip(self, update))]
    pub async fn process_update(&mut self, update: &QuantumUpdate) -> Result<LearningOutput, LearningError> {
        // Apply quantum update to weights
        let gradients = update.gradient.clone();
        self.update_weights(&Array2::from_shape_vec(
            self.state.weights.raw_dim(),
            gradients.clone(),
        ).map_err(|e| LearningError::TrainingError(e.to_string()))?)?;

        // Compute loss
        let loss = self.state.stats.current_loss;

        Ok(LearningOutput {
            loss,
            gradients,
        })
    }

    /// Perform training step
    #[instrument(skip(self, input, target))]
    pub async fn train_step(
        &mut self,
        input: &Array1<Complex64>,
        target: &Array1<Complex64>,
    ) -> Result<f64, LearningError> {
        // Compute forward pass
        let output = self.forward(input)?;
        
        // Compute loss and gradients
        let (loss, base_gradients) = self.compute_gradients(&output, target)?;

        // Apply quantum corrections if enabled
        let final_gradients = if self.config.enable_quantum {
            self.apply_quantum_corrections(&base_gradients).await?
        } else {
            base_gradients
        };

        // Update weights with momentum
        self.update_weights(&final_gradients)?;

        // Update statistics
        self.update_stats(loss);

        Ok(loss)
    }

    /// Forward pass through the network
    fn forward(&self, input: &Array1<Complex64>) -> Result<Array1<Complex64>, LearningError> {
        // Apply weight matrix
        let mut output = self.state.weights.dot(input);

        // Apply nonlinearity (complex tanh)
        output.mapv_inplace(|x| {
            let r = x.norm();
            let theta = x.arg();
            Complex64::from_polar(r.tanh(), theta)
        });

        Ok(output)
    }

    /// Compute loss and gradients
    fn compute_gradients(
        &self,
        output: &Array1<Complex64>,
        target: &Array1<Complex64>,
    ) -> Result<(f64, Array2<Complex64>), LearningError> {
        // Compute error
        let error = target - output;
        
        // Compute loss (MSE in complex space)
        let loss = error.iter()
            .map(|x| x.norm_sqr())
            .sum::<f64>() / (error.len() as f64);

        // Compute gradients
        let mut gradients = Array2::zeros(self.state.weights.raw_dim());
        for i in 0..gradients.shape()[0] {
            for j in 0..gradients.shape()[1] {
                gradients[[i, j]] = -error[i] * output[j].conj();
            }
        }

        // Add L2 regularization
        if self.config.l2_reg > 0.0 {
            let reg_term = &self.state.weights * self.config.l2_reg;
            gradients = gradients + &reg_term;
        }

        Ok((loss, gradients))
    }

    /// Apply quantum corrections to gradients
    async fn apply_quantum_corrections(
        &self,
        gradients: &Array2<Complex64>,
    ) -> Result<Array2<Complex64>, LearningError> {
        // Convert gradients to ComplexNeuralTarget format
        let target = ComplexNeuralTarget(gradients.iter().cloned().collect());

        // Convert to NeuralInput for quantum interface
        let input = NeuralInput {
            values: target.0.iter().map(|c| c.norm()).collect(),
            timestamp: SystemTime::now(),
            metadata: StateMetadata::default(),
        };

        // Get quantum corrections through learning update
        let update = self.quantum
            .as_ref()
            .process_input(&input)
            .await
            .map_err(|e| LearningError::QuantumError(e.to_string()))?;

        // Convert update gradient back to Array2
        let update_array = Array2::from_shape_vec(
            gradients.raw_dim(),
            update.gradient
        ).map_err(|e| LearningError::TrainingError(e.to_string()))?;

        // Apply corrections from the quantum update
        Ok(gradients + &update_array)
    }

    /// Update weights using momentum
    fn update_weights(&mut self, gradients: &Array2<Complex64>) -> Result<(), LearningError> {
        // Update velocities
        self.state.velocities.mapv_inplace(|v| v * self.config.momentum);
        self.state.velocities = &self.state.velocities - &(gradients * self.config.learning_rate);

        // Update weights
        let weights = &mut self.state.weights;
        *weights = &*weights + &self.state.velocities;

        Ok(())
    }

    /// Update learning statistics
    fn update_stats(&mut self, loss: f64) {
        let stats = &mut self.state.stats;
        stats.total_steps += 1;
        stats.current_loss = loss;

        // Update moving average
        if stats.total_steps == 1 {
            stats.avg_loss = loss;
        } else {
            stats.avg_loss = 0.9 * stats.avg_loss + 0.1 * loss;
        }

        // Update best loss
        if loss < stats.best_loss {
            stats.best_loss = loss;
            stats.steps_no_improve = 0;
        } else {
            stats.steps_no_improve += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_input() -> Array1<Complex64> {
        Array1::from_vec(vec![
            Complex64::new(0.5, 0.0),
            Complex64::new(-0.3, 0.2),
        ])
    }

    fn create_test_target() -> Array1<Complex64> {
        Array1::from_vec(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 1.0),
        ])
    }

    #[tokio::test]
    async fn test_initialization() -> Result<(), LearningError> {
        let config = LearningConfig::default();
        let mut system = LearningSystem::new(config);
        system.initialize()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_forward_pass() -> Result<(), LearningError> {
        let config = LearningConfig::default();
        let mut system = LearningSystem::new(config);
        system.initialize()?;

        let input = create_test_input();
        let output = system.forward(&input)?;
        assert_eq!(output.len(), system.config.hidden_dim);
        Ok(())
    }

    #[tokio::test]
    async fn test_training_step() -> Result<(), LearningError> {
        let config = LearningConfig::default();
        let mut system = LearningSystem::new(config);
        system.initialize()?;

        let input = create_test_input();
        let target = create_test_target();
        let loss = system.train_step(&input, &target).await?;
        assert!(loss >= 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_reset() -> Result<(), LearningError> {
        let config = LearningConfig::default();
        let mut system = LearningSystem::new(config);
        system.initialize()?;

        let input = create_test_input();
        let target = create_test_target();
        system.train_step(&input, &target).await?;

        system.initialize()?;
        assert_eq!(system.state.stats.total_steps, 0);
        Ok(())
    }
}