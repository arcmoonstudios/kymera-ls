// state

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use thiserror::Error;
use tracing::{debug, instrument};

/// State management errors
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("Update error: {0}")]
    UpdateError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

/// Reasoning state
#[derive(Debug, Clone)]
pub struct ReasoningState {
    /// Hidden state
    hidden: Array1<Complex64>,
    /// Memory state
    memory: Array1<Complex64>,
    /// Attention state
    attention: Option<Array1<f64>>,
}

impl ReasoningState {
    /// Create new reasoning state
    pub fn new(hidden_dim: usize) -> Self {
        Self {
            hidden: Array1::zeros(hidden_dim),
            memory: Array1::zeros(hidden_dim),
            attention: None,
        }
    }

    /// Get hidden state
    pub fn hidden(&self) -> &Array1<Complex64> {
        &self.hidden
    }

    /// Get memory state
    pub fn memory(&self) -> &Array1<Complex64> {
        &self.memory
    }

    /// Get attention state
    pub fn attention(&self) -> Option<&Array1<f64>> {
        self.attention.as_ref()
    }
}

/// State manager implementation
#[derive(Debug)]
pub struct StateManager {
    /// State dimension
    state_dim: usize,
    /// Current state
    current_state: ReasoningState,
    /// State transition matrix
    transition: Array2<Complex64>,
    /// Input projection matrix
    projection: Array2<Complex64>,
}

impl StateManager {
    /// Create new state manager
    #[instrument(skip(state_dim))]
    pub fn new(state_dim: usize) -> Result<Self, StateError> {
        let current_state = ReasoningState::new(state_dim);
        
        // Initialize transition matrix as identity + small random perturbations
        let mut transition = Array2::eye(state_dim);
        transition.mapv_inplace(|x| {
            x + Complex64::new(
                rand::random::<f64>() * 0.1 - 0.05,
                rand::random::<f64>() * 0.1 - 0.05
            )
        });

        // Initialize projection matrix with random weights
        let mut projection = Array2::zeros((state_dim, state_dim));
        projection.mapv_inplace(|_| {
            Complex64::new(
                rand::random::<f64>() * 2.0 - 1.0,
                rand::random::<f64>() * 2.0 - 1.0
            )
        });

        debug!("Initialized state manager");

        Ok(Self {
            state_dim,
            current_state,
            transition,
            projection,
        })
    }

    /// Compute state update
    #[instrument(skip(self, input, hidden))]
    pub fn compute_update(
        &mut self,
        input: &Array1<Complex64>,
        hidden: &Array1<Complex64>,
    ) -> Result<Array1<Complex64>, StateError> {
        if input.len() != self.state_dim {
            return Err(StateError::InvalidState(format!(
                "Expected input dimension {}, got {}",
                self.state_dim,
                input.len()
            )));
        }

        // Project input
        let input_contribution = self.projection.dot(input);

        // Apply state transition
        let state_contribution = self.transition.dot(hidden);

        // Combine contributions
        let mut update = input_contribution + state_contribution;

        // Apply nonlinearity (complex tanh)
        update.mapv_inplace(|x| {
            let r = x.norm();
            let theta = x.arg();
            Complex64::from_polar(r.tanh(), theta)
        });

        Ok(update)
    }

    /// Update state with attention
    pub fn update_attention(&mut self, attention: Option<Array1<f64>>) -> Result<(), StateError> {
        if let Some(att) = &attention {
            if att.len() != self.state_dim {
                return Err(StateError::InvalidState(format!(
                    "Expected attention dimension {}, got {}",
                    self.state_dim,
                    att.len()
                )));
            }
        }
        self.current_state.attention = attention;
        Ok(())
    }

    /// Get current state
    pub fn current_state(&self) -> &ReasoningState {
        &self.current_state
    }

    /// Reset state
    pub fn reset(&mut self) -> Result<(), StateError> {
        self.current_state = ReasoningState::new(self.state_dim);
        Ok(())
    }

    /// Update transition dynamics
    pub fn update_dynamics(&mut self, learning_rate: f64) -> Result<(), StateError> {
        // Add small random updates to transition matrix
        self.transition.mapv_inplace(|x| {
            x + Complex64::new(
                rand::random::<f64>() * learning_rate - learning_rate / 2.0,
                rand::random::<f64>() * learning_rate - learning_rate / 2.0
            )
        });

        // Normalize to prevent instability
        let norm = self.transition.iter().map(|x| x.norm()).sum::<f64>().sqrt();
        if norm > 0.0 {
            self.transition.mapv_inplace(|x| x / norm);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn create_test_input() -> Array1<Complex64> {
        Array1::from_vec(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 1.0),
            Complex64::new(-1.0, -1.0),
        ])
    }

    #[test]
    fn test_initialization() -> Result<(), StateError> {
        let state_dim = 3;
        let manager = StateManager::new(state_dim)?;

        assert_eq!(manager.current_state.hidden.len(), state_dim);
        assert_eq!(manager.transition.shape(), &[state_dim, state_dim]);
        assert_eq!(manager.projection.shape(), &[state_dim, state_dim]);

        Ok(())
    }

    #[test]
    fn test_state_update() -> Result<(), StateError> {
        let state_dim = 3;
        let mut manager = StateManager::new(state_dim)?;

        let input = create_test_input();
        let hidden = Array1::zeros(state_dim);
        let update = manager.compute_update(&input, &hidden)?;

        assert_eq!(update.len(), state_dim);
        assert!(update.iter().any(|x| x.norm() > 0.0));

        Ok(())
    }

    #[test]
    fn test_attention_update() -> Result<(), StateError> {
        let state_dim = 3;
        let mut manager = StateManager::new(state_dim)?;

        let attention = Array1::from_vec(vec![0.5, 0.3, 0.2]);
        manager.update_attention(Some(attention.clone()))?;

        assert!(manager.current_state.attention.is_some());
        assert_eq!(
            manager.current_state.attention.as_ref().unwrap(),
            &attention
        );

        Ok(())
    }

    #[test]
    fn test_dynamics_update() -> Result<(), StateError> {
        let state_dim = 3;
        let mut manager = StateManager::new(state_dim)?;

        let old_transition = manager.transition.clone();
        manager.update_dynamics(0.1)?;

        // Transition matrix should change but maintain reasonable values
        assert!(manager.transition.iter().zip(old_transition.iter()).any(|(a, b)| a != b));
        assert!(manager.transition.iter().all(|x| x.norm() <= 1.0));

        Ok(())
    }

    #[test]
    fn test_reset() -> Result<(), StateError> {
        let state_dim = 3;
        let mut manager = StateManager::new(state_dim)?;

        let attention = Array1::from_vec(vec![0.5, 0.3, 0.2]);
        manager.update_attention(Some(attention))?;
        manager.reset()?;

        assert!(manager.current_state.attention.is_none());
        for x in manager.current_state.hidden.iter() {
            assert_relative_eq!(x.norm(), 0.0, epsilon = 1e-10);
        }

        Ok(())
    }
}
