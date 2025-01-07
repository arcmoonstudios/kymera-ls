// state

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, instrument};
use std::ops::{Add, AddAssign, SubAssign};

/// State management errors
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("State update error: {0}")]
    UpdateError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

/// State management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    /// Maximum state history length
    pub history_length: usize,
    /// State dimension
    pub state_dim: usize,
    /// Enable state compression
    pub enable_compression: bool,
    /// Compression threshold
    pub compression_threshold: f64,
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            history_length: 100,
            state_dim: 100,
            enable_compression: false,
            compression_threshold: 0.01,
        }
    }
}

/// State manager implementation
#[derive(Debug)]
pub struct StateManager {
    /// Configuration
    config: StateConfig,
    /// Current state
    current_state: Array1<Complex64>,
    /// State history
    history: Vec<Array1<Complex64>>,
    /// State statistics
    statistics: StateStatistics,
}

/// State statistics
#[derive(Debug, Default)]
struct StateStatistics {
    /// Mean state
    mean: Option<Array1<f64>>,
    /// State variance
    variance: Option<Array1<f64>>,
    /// Number of updates
    updates: usize,
}

impl StateManager {
    /// Create new state manager
    #[instrument(skip(config))]
    pub fn new(config: StateConfig) -> Result<Self, StateError> {
        let current_state = Array1::zeros(config.state_dim)
            .mapv(|_: f64| Complex64::new(0.0, 0.0));

        debug!("Initialized state manager");

        Ok(Self {
            config: config.clone(),
            current_state,
            history: Vec::with_capacity(config.history_length),
            statistics: StateStatistics::default(),
        })
    }

    /// Update state with new reservoir state
    #[instrument(skip(self, reservoir_state))]
    pub fn update_state(&mut self, reservoir_state: &Array1<Complex64>) -> Result<(), StateError> {
        if reservoir_state.len() != self.config.state_dim {
            return Err(StateError::InvalidState(format!(
                "Expected state dimension {}, got {}",
                self.config.state_dim,
                reservoir_state.len()
            )));
        }

        // Update current state
        self.current_state.assign(reservoir_state);

        // Update history
        if self.history.len() >= self.config.history_length {
            self.history.remove(0);
        }
        self.history.push(reservoir_state.clone());

        // Update statistics
        self.update_statistics(reservoir_state)?;

        // Perform compression if enabled
        if self.config.enable_compression {
            self.compress_history()?;
        }

        Ok(())
    }

    /// Update state statistics
    fn update_statistics(&mut self, state: &Array1<Complex64>) -> Result<(), StateError> {
        let real_state = state.mapv(|x| x.norm());
        
        if let Some(mean) = &mut self.statistics.mean {
            // Online mean update
            let n = self.statistics.updates as f64;
            mean.mapv_inplace(|x: f64| x * (n / (n + 1.0)));
            let scaled_state = real_state.mapv(|x: f64| x / (n + 1.0));
            mean.add_assign(&scaled_state);

            // Online variance update
            if let Some(var) = &mut self.statistics.variance {
                let mean_scaled = mean.mapv(|x| x * (n + 1.0) / n);
                let delta = real_state.to_owned() - mean.to_owned();
                let delta2 = real_state.to_owned() - mean_scaled;
                var.mapv_inplace(|x| x * (n - 1.0) / n);
                var.add_assign(&(&delta * &delta2).mapv(|x| x / n));
            }
        } else {
            // Initialize statistics
            self.statistics.mean = Some(real_state.clone());
            self.statistics.variance = Some(Array1::zeros(self.config.state_dim));
        }

        self.statistics.updates += 1;
        Ok(())
    }

    /// Compress state history using PCA-like approach
    fn compress_history(&mut self) -> Result<(), StateError> {
        if self.history.len() < 2 {
            return Ok(());
        }

        // Convert history to real matrix
        let mut matrix = Array2::zeros((self.history.len(), self.config.state_dim));
        for (i, state) in self.history.iter().enumerate() {
            matrix.row_mut(i).assign(&state.mapv(|x: Complex64| x.norm()));
        }

        // Center the data
        let mean = matrix.mean_axis(ndarray::Axis(0))
            .ok_or_else(|| StateError::UpdateError("Failed to compute mean".into()))?;
        for mut row in matrix.rows_mut() {
            row.sub_assign(&mean);
        }

        // Compute correlation matrix
        let corr = matrix.t().dot(&matrix);
        let norm = (self.history.len() as f64).sqrt();
        let corr = corr.mapv(|x| x / norm);

        // Find principal components
        let (eigenvalues, eigenvectors) = Self::power_iteration(&corr, 3)
            .map_err(|e| StateError::UpdateError(format!("Failed to compute eigenvectors: {}", e)))?;

        // Keep only significant components
        let total_variance: f64 = eigenvalues.iter().sum();
        let significant: Vec<_> = eigenvalues.iter()
            .zip(eigenvectors.axis_iter(ndarray::Axis(1)))
            .filter(|&(eval, _)| eval / total_variance > self.config.compression_threshold)
            .map(|(_, evec)| evec.to_owned())
            .collect();

        if significant.is_empty() {
            return Ok(());
        }

        // Project data onto significant components
        let projection = Array2::from_shape_vec(
            (significant.len(), self.config.state_dim),
            significant.into_iter().flatten().collect(),
        ).map_err(|e| StateError::UpdateError(format!("Failed to create projection matrix: {}", e)))?;

        // Update history with compressed states
        self.history = matrix
            .dot(&projection.t())
            .dot(&projection)
            .rows()
            .into_iter()
            .map(|row| {
                row.add(&mean)
                    .mapv(|x| Complex64::new(x, 0.0))
                    .to_owned()
            })
            .collect();

        Ok(())
    }

    /// Power iteration method for eigendecomposition
    fn power_iteration(matrix: &Array2<f64>, n_components: usize) -> Result<(Vec<f64>, Array2<f64>), StateError> {
        let size = matrix.nrows();
        let mut eigenvalues = Vec::with_capacity(n_components);
        let mut eigenvectors = Array2::zeros((size, n_components));
        let mut residual = matrix.to_owned();

        for k in 0..n_components {
            let (eval, evec) = Self::largest_eigenpair(&residual)?;
            eigenvalues.push(eval);
            eigenvectors.column_mut(k).assign(&evec);

            // Deflate matrix
            let outer = evec.clone().into_shape((size, 1)).unwrap()
                .dot(&evec.clone().into_shape((1, size)).unwrap())
                .mapv(|x| x * eval);
            residual -= &outer;
        }

        Ok((eigenvalues, eigenvectors))
    }

    /// Find largest eigenpair using power iteration
    fn largest_eigenpair(matrix: &Array2<f64>) -> Result<(f64, Array1<f64>), StateError> {
        let size = matrix.nrows();
        let max_iter = 100;
        let tolerance = 1e-6;

        let mut v = Array1::zeros(size).mapv(|_: f64| 1.0);
        let mut lambda = 0.0;

        for _ in 0..max_iter {
            let mut v_next = matrix.dot(&v);
            let norm = v_next.mapv(|x| x * x).sum().sqrt();
            
            if norm < 1e-10 {
                return Ok((0.0, v));
            }

            v_next.mapv_inplace(|x| x / norm);
            let lambda_next = v_next.dot(&matrix.dot(&v_next));

            if (lambda_next - lambda).abs() < tolerance {
                return Ok((lambda_next, v_next));
            }

            lambda = lambda_next;
            v = v_next;
        }

        Ok((lambda, v))
    }

    /// Get current state
    pub fn current_state(&self) -> Result<Array1<Complex64>, StateError> {
        Ok(self.current_state.clone())
    }

    /// Get state history
    pub fn history(&self) -> Result<&[Array1<Complex64>], StateError> {
        Ok(&self.history)
    }

    /// Get state statistics
    pub fn statistics(&self) -> Result<(Option<&Array1<f64>>, Option<&Array1<f64>>), StateError> {
        Ok((self.statistics.mean.as_ref(), self.statistics.variance.as_ref()))
    }

    /// Reset state manager
    pub fn reset(&mut self) -> Result<(), StateError> {
        self.current_state.fill(Complex64::new(0.0, 0.0));
        self.history.clear();
        self.statistics = StateStatistics::default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn create_test_config() -> StateConfig {
        StateConfig {
            history_length: 5,
            state_dim: 3,
            enable_compression: true,
            compression_threshold: 0.1,
        }
    }

    fn create_test_state() -> Array1<Complex64> {
        Array1::from_vec(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 1.0),
            Complex64::new(-1.0, -1.0),
        ])
    }

    #[test]
    fn test_initialization() -> Result<(), StateError> {
        let config = create_test_config();
        let manager = StateManager::new(config.clone())?;

        assert_eq!(manager.current_state.len(), config.state_dim);
        assert!(manager.history.is_empty());
        assert!(manager.statistics.mean.is_none());

        Ok(())
    }

    #[test]
    fn test_state_update() -> Result<(), StateError> {
        let config = create_test_config();
        let mut manager = StateManager::new(config)?;

        let state = create_test_state();
        manager.update_state(&state)?;

        assert_eq!(manager.current_state, state);
        assert_eq!(manager.history.len(), 1);
        assert!(manager.statistics.mean.is_some());

        Ok(())
    }

    #[test]
    fn test_history_limit() -> Result<(), StateError> {
        let config = create_test_config();
        let mut manager = StateManager::new(config.clone())?;

        let state = create_test_state();
        for _ in 0..config.history_length + 2 {
            manager.update_state(&state)?;
        }

        assert_eq!(manager.history.len(), config.history_length);
        Ok(())
    }

    #[test]
    fn test_compression() -> Result<(), StateError> {
        let mut config = create_test_config();
        config.enable_compression = true;
        let mut manager = StateManager::new(config)?;

        let state1 = create_test_state();
        let state2 = state1.mapv(|x| x * 2.0);
        let state3 = state1.mapv(|x| x * 3.0);

        manager.update_state(&state1)?;
        manager.update_state(&state2)?;
        manager.update_state(&state3)?;

        // Compression should preserve the linear relationship
        let history = manager.history()?;
        assert!(!history.is_empty());

        Ok(())
    }

    #[test]
    fn test_reset() -> Result<(), StateError> {
        let config = create_test_config();
        let mut manager = StateManager::new(config)?;

        let state = create_test_state();
        manager.update_state(&state)?;
        manager.reset()?;

        assert!(manager.history.is_empty());
        assert!(manager.statistics.mean.is_none());
        for x in manager.current_state.iter() {
            assert_relative_eq!(x.norm(), 0.0, epsilon = 1e-10);
        }

        Ok(())
    }
}
