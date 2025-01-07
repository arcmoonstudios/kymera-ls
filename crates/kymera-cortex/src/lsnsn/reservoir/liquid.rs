// liquid


use ndarray::{Array1, Array2, ArrayView1};
use num_complex::Complex64;
use rand_distr::Normal;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, instrument};


/// Liquid computing errors
#[derive(Error, Debug)]
pub enum LiquidError {
    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Liquid computing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidConfig {
    /// Size of the reservoir
    pub reservoir_size: usize,
    /// Input dimension
    pub input_dim: usize,
    /// Spectral radius
    pub spectral_radius: f64,
    /// Input scaling
    pub input_scaling: f64,
    /// Connectivity density (0-1)
    pub connectivity: f64,
    /// Leaking rate
    pub leaking_rate: f64,
    /// Random seed
    pub seed: Option<u64>,
}

impl Default for LiquidConfig {
    fn default() -> Self {
        Self {
            reservoir_size: 100,
            input_dim: 1,
            spectral_radius: 0.9,
            input_scaling: 1.0,
            connectivity: 0.1,
            leaking_rate: 0.3,
            seed: None,
        }
    }
}

/// Liquid reservoir implementation
#[derive(Debug)]
pub struct LiquidReservoir {
    /// Configuration
    config: LiquidConfig,
    /// Reservoir state
    state: Array1<Complex64>,
    /// Reservoir weights
    weights: Array2<Complex64>,
    /// Input weights
    input_weights: Array2<f64>,
}

impl LiquidReservoir {
    /// Create new liquid reservoir
    #[instrument(skip(config))]
    pub fn new(config: LiquidConfig) -> Result<Self, LiquidError> {
        let mut rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        // Initialize reservoir state
        let state = Array1::zeros(config.reservoir_size)
            .mapv(|_: f64| Complex64::new(0.0, 0.0));

        // Initialize reservoir weights with sparse random connections
        let weights = Self::initialize_reservoir_weights(
            config.reservoir_size,
            config.connectivity,
            config.spectral_radius,
            &mut rng,
        )?;

        // Initialize input weights
        let input_weights = Self::initialize_input_weights(
            config.reservoir_size,
            config.input_dim,
            config.input_scaling,
            &mut rng,
        )?;

        debug!("Initialized liquid reservoir");

        Ok(Self {
            config,
            state,
            weights,
            input_weights,
        })
    }

    /// Initialize reservoir weights
    fn initialize_reservoir_weights(
        size: usize,
        connectivity: f64,
        spectral_radius: f64,
        rng: &mut StdRng,
    ) -> Result<Array2<Complex64>, LiquidError> {
        // Generate sparse random matrix
        let dist = Normal::new(0.0, 1.0)
            .map_err(|e| LiquidError::InitError(format!("Failed to create normal distribution: {}", e)))?;
        
        let mut weights = Array2::zeros((size, size));
        for i in 0..size {
            for j in 0..size {
                if Uniform::new(0.0, 1.0).sample(rng) < connectivity {
                    weights[[i, j]] = Complex64::new(dist.sample(rng), dist.sample(rng));
                }
            }
        }

        // Scale weights to desired spectral radius
        let max_eigenvalue = Self::estimate_spectral_radius(&weights)
            .map_err(|e| LiquidError::InitError(format!("Failed to estimate spectral radius: {}", e)))?;

        if max_eigenvalue.norm() > 0.0 {
            weights.mapv_inplace(|x| x * (spectral_radius / max_eigenvalue.norm()));
        }

        Ok(weights)
    }

    /// Initialize input weights
    fn initialize_input_weights(
        reservoir_size: usize,
        input_dim: usize,
        input_scaling: f64,
        rng: &mut StdRng,
    ) -> Result<Array2<f64>, LiquidError> {
        let dist = Normal::new(0.0, input_scaling)
            .map_err(|e| LiquidError::InitError(format!("Failed to create normal distribution: {}", e)))?;

        let mut weights = Array2::zeros((reservoir_size, input_dim));
        for i in 0..reservoir_size {
            for j in 0..input_dim {
                weights[[i, j]] = dist.sample(rng);
            }
        }

        Ok(weights)
    }

    /// Estimate spectral radius using power iteration method
    fn estimate_spectral_radius(matrix: &Array2<Complex64>) -> Result<Complex64, LiquidError> {
        let size = matrix.nrows();
        let max_iter = 100;
        let tolerance = 1e-6;

        let mut v: Array1<Complex64> = Array1::zeros(size).mapv(|_: f64| Complex64::new(1.0, 0.0));
        let mut lambda = Complex64::new(0.0, 0.0);

        for _ in 0..max_iter {
            let mut v_next: Array1<Complex64> = matrix.dot(&v);
            let norm = v_next.mapv(|x: Complex64| x.norm()).sum().sqrt();
            
            if norm < 1e-10 {
                return Ok(Complex64::new(0.0, 0.0));
            }

            v_next.mapv_inplace(|x| x / Complex64::new(norm, 0.0));
            let lambda_next = v_next.dot(&matrix.dot(&v_next));

            if (lambda_next - lambda).norm() < tolerance {
                return Ok(lambda_next);
            }

            lambda = lambda_next;
            v = v_next;
        }

        Ok(lambda)
    }

    /// Update reservoir state with new input
    #[instrument(skip(self, input))]
    pub fn update(&mut self, input: ArrayView1<f64>) -> Result<(), LiquidError> {
        if input.len() != self.config.input_dim {
            return Err(LiquidError::InvalidInput(format!(
                "Expected input dimension {}, got {}",
                self.config.input_dim,
                input.len()
            )));
        }

        // Compute input contribution
        let input_term = self.input_weights.dot(&input);

        // Compute reservoir contribution
        let reservoir_term = self.weights.dot(&self.state);

        // Update state with leaky integration
        let new_state = reservoir_term + input_term.mapv(|x| Complex64::new(x, 0.0));
        let indices: Vec<_> = self.state.iter().enumerate()
            .map(|(i, _)| i)
            .collect();

        self.state.mapv_inplace(|x: Complex64| {
            let idx = indices[x.re.abs() as usize % indices.len()];
            x * (1.0 - self.config.leaking_rate) + 
            new_state[idx] * self.config.leaking_rate
        });

        // Apply nonlinearity (tanh)
        self.state.mapv_inplace(|x| {
            let r = x.norm();
            let theta = x.arg();
            Complex64::from_polar(r.tanh(), theta)
        });

        Ok(())
    }

    /// Get current reservoir state
    pub fn state(&self) -> Result<Array1<Complex64>, LiquidError> {
        Ok(self.state.clone())
    }

    /// Reset reservoir state
    pub fn reset(&mut self) -> Result<(), LiquidError> {
        self.state.fill(Complex64::new(0.0, 0.0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn create_test_config() -> LiquidConfig {
        LiquidConfig {
            reservoir_size: 10,
            input_dim: 2,
            spectral_radius: 0.9,
            input_scaling: 1.0,
            connectivity: 0.3,
            leaking_rate: 0.3,
            seed: Some(42),
        }
    }

    #[test]
    fn test_initialization() -> Result<(), LiquidError> {
        let config = create_test_config();
        let reservoir = LiquidReservoir::new(config.clone())?;

        assert_eq!(reservoir.state.len(), config.reservoir_size);
        assert_eq!(reservoir.weights.shape(), &[config.reservoir_size, config.reservoir_size]);
        assert_eq!(reservoir.input_weights.shape(), &[config.reservoir_size, config.input_dim]);

        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), LiquidError> {
        let config = create_test_config();
        let mut reservoir = LiquidReservoir::new(config)?;

        let input = Array1::from_vec(vec![0.5, -0.3]);
        reservoir.update(input.view())?;

        // State should be non-zero after update
        assert!(reservoir.state.iter().any(|&x| x.norm() > 0.0));

        Ok(())
    }

    #[test]
    fn test_reset() -> Result<(), LiquidError> {
        let config = create_test_config();
        let mut reservoir = LiquidReservoir::new(config)?;

        let input = Array1::from_vec(vec![0.5, -0.3]);
        reservoir.update(input.view())?;
        reservoir.reset()?;

        // All state values should be zero after reset
        for x in reservoir.state.iter() {
            assert_relative_eq!(x.norm(), 0.0, epsilon = 1e-10);
        }

        Ok(())
    }

    #[test]
    fn test_spectral_radius() -> Result<(), LiquidError> {
        let config = create_test_config();
        let reservoir = LiquidReservoir::new(config.clone())?;

        let estimated_radius = LiquidReservoir::estimate_spectral_radius(&reservoir.weights)?;
        assert_relative_eq!(
            estimated_radius.norm(),
            config.spectral_radius,
            epsilon = 0.1
        );

        Ok(())
    }
}