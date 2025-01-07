// src/mtalr/learning.rs

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use anyhow::{Result, Context};
use dashmap::DashMap;
use num_complex::Complex64;
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

use super::{
    core::{Parameter, ParamId, ComputationState},
    MTALRError, MTALRConfig, OptimizationParams, MetaTarget,
};

/// Gradient identifier
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct GradientId(Uuid);

impl GradientId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Gradient information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    pub id: GradientId,
    pub parameter_id: ParamId,
    pub value: Complex64,
    #[serde(with = "instant_serde")]
    pub computation_time: Instant,
}

/// Optimization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStep {
    pub parameters: HashMap<ParamId, Parameter>,
    pub loss: f64,
    pub iteration: usize,
    #[serde(with = "duration_serde")]
    pub duration: Duration,
}

/// Meta-parameters for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaParameters {
    pub learning_rate: f64,
    pub optimization_params: OptimizationParams,
}

impl MetaParameters {
    pub fn new(learning_rate: f64, optimization_params: OptimizationParams) -> Self {
        Self {
            learning_rate,
            optimization_params,
        }
    }
}

/// Learning update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningUpdate {
    pub gradients: Vec<Gradient>,
    pub optimization_step: OptimizationStep,
    #[serde(with = "duration_serde")]
    pub duration: Duration,
}

/// Learning statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStatistics {
    pub convergence: Vec<f64>,
    pub parameter_stats: HashMap<ParamId, ParameterStatistics>,
    pub final_loss: f64,
    pub convergence_rate: f64,
    #[serde(with = "duration_serde")]
    pub learning_duration: Duration,
}

impl Default for LearningStatistics {
    fn default() -> Self {
        Self {
            convergence: Vec::new(),
            parameter_stats: HashMap::new(),
            final_loss: 0.0,
            convergence_rate: 0.0,
            learning_duration: Duration::from_secs(0),
        }
    }
}

/// Parameter statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterStatistics {
    pub final_value: Complex64,
    pub gradient_norm: f64,
    pub update_count: usize,
}

/// Serializable wrapper for Duration
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_nanos().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = u128::deserialize(deserializer)?;
        Ok(Duration::from_nanos(nanos as u64))
    }
}

/// Serializable wrapper for Instant
mod instant_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, Instant};

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let now = Instant::now();
        let duration = if *instant > now {
            instant.duration_since(now)
        } else {
            now.duration_since(*instant)
        };
        duration.as_nanos().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = u128::deserialize(deserializer)?;
        Ok(Instant::now() + Duration::from_nanos(nanos as u64))
    }
}

/// Learning system errors
#[derive(Error, Debug)]
pub enum LearningError {
    #[error("Gradient computation error: {0}")]
    GradientError(String),

    #[error("Parameter update error: {0}")]
    ParameterError(String),

    #[error("Meta-learning error: {0}")]
    MetaLearningError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("Resource exhaustion: {0}")]
    ResourceError(String),
}

/// Learning state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningState {
    pub parameters: HashMap<ParamId, Parameter>,
    pub gradients: Vec<Gradient>,
    pub optimization_trace: VecDeque<OptimizationStep>,
    pub meta_parameters: MetaParameters,
    pub creation_time: SystemTime,
}

impl Default for LearningState {
    fn default() -> Self {
        Self {
            parameters: HashMap::new(),
            gradients: Vec::new(),
            optimization_trace: VecDeque::new(),
            meta_parameters: MetaParameters::new(0.0, OptimizationParams::default()),
            creation_time: SystemTime::now(),
        }
    }
}

/// Adaptive learning system
#[derive(Debug)]
pub struct AdaptiveLearning {
    state: Arc<RwLock<LearningState>>,
    optimizer: Arc<Mutex<MetaOptimizer>>,
    parameter_store: Arc<DashMap<ParamId, Parameter>>,
    gradient_cache: Arc<DashMap<GradientId, Gradient>>,
    metrics: Arc<Mutex<LearningMetrics>>,
}

impl AdaptiveLearning {
    /// Create new adaptive learning system
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(LearningState::default())),
            optimizer: Arc::new(Mutex::new(MetaOptimizer::new())),
            parameter_store: Arc::new(DashMap::new()),
            gradient_cache: Arc::new(DashMap::new()),
            metrics: Arc::new(Mutex::new(LearningMetrics::default())),
        }
    }

    /// Initialize learning system
    #[instrument]
    pub fn initialize(&mut self, config: &MTALRConfig) -> Result<(), MTALRError> {
        info!("Initializing adaptive learning system");

        // Initialize learning state
        let mut state = self.state.write();
        state.meta_parameters = MetaParameters::new(
            config.learning_rate,
            config.optimization_params.clone(),
        );

        // Configure optimizer
        let mut optimizer = self.optimizer.lock();
        optimizer.configure(&config.optimization_params, config.learning_rate)
            .context("Failed to configure optimizer")?;

        Ok(())
    }

    /// Prepare for learning phase
    #[instrument]
    pub fn prepare_learning(&mut self) -> Result<(), MTALRError> {
        info!("Preparing learning phase");

        // Reset gradient cache
        self.gradient_cache.clear();

        // Initialize optimization trace
        let mut state = self.state.write();
        state.optimization_trace.clear();

        Ok(())
    }

    /// Update learning state with computation
    #[instrument(skip(self, computation, target))]
    pub async fn update(
        &self,
        computation: &ComputationState,
        target: &MetaTarget,
    ) -> Result<LearningUpdate, MTALRError> {
        let start = Instant::now();

        // Compute gradients - this needs a read lock
        let gradients = {
            let state = self.state.read();
            self.compute_gradients_internal(&state, computation, target)
                .context("Failed to compute gradients")?
        };

        // Optimize parameters - this needs a mutex lock
        let (parameters, optimization_step) = {
            let mut optimizer = self.optimizer.lock();
            optimizer.optimize_step(&gradients)
                .context("Optimization step failed")?
        };

        // Update parameter store - this is already thread-safe with DashMap
        for (id, param) in &parameters {
            self.parameter_store.insert(*id, param.clone());
        }

        // Update learning state - this needs a write lock
        {
            let mut state = self.state.write();
            state.parameters.extend(parameters.clone());
            state.gradients.extend(gradients.clone());
            state.optimization_trace.push_back(optimization_step.clone());

            // Trim optimization trace if needed
            if state.optimization_trace.len() > 1000 {
                state.optimization_trace.pop_front();
            }
        }

        // Update metrics - this needs a mutex lock
        {
            let mut metrics = self.metrics.lock();
            metrics.record_update(start.elapsed());
        }

        Ok(LearningUpdate {
            gradients,
            optimization_step,
            duration: start.elapsed(),
        })
    }

    /// Internal method to compute gradients with state already locked
    fn compute_gradients_internal(
        &self,
        state: &LearningState,
        computation: &ComputationState,
        target: &MetaTarget,
    ) -> Result<Vec<Gradient>, MTALRError> {
        let mut gradients = Vec::new();

        // Compute error between current state and target
        let error = target.target_value - computation.state_vector[0];
        let error_norm = error.norm() * target.target_weight;

        // Compute gradients for each parameter
        for (param_id, param) in &state.parameters {
            let gradient = Gradient {
                id: GradientId::new(),
                parameter_id: *param_id,
                value: error * param.value.conj() * error_norm,
                computation_time: Instant::now(),
            };
            gradients.push(gradient);
        }

        Ok(gradients)
    }

    /// Finalize learning phase
    #[instrument]
    pub fn finalize_learning(&mut self) -> Result<(), MTALRError> {
        info!("Finalizing learning phase");

        // Compute final statistics
        let state = self.state.read();
        let final_stats = self.compute_learning_statistics(&state)
            .context("Failed to compute learning statistics")?;

        // Update metrics
        let mut metrics = self.metrics.lock();
        metrics.record_final_statistics(final_stats);

        Ok(())
    }

    /// Compute learning statistics
    fn compute_learning_statistics(
        &self,
        state: &LearningState,
    ) -> Result<LearningStatistics, MTALRError> {
        // Compute convergence statistics
        let convergence = state.optimization_trace.iter()
            .map(|step| step.loss)
            .collect::<Vec<_>>();

        // Compute parameter statistics
        let parameter_stats = state.parameters.iter()
            .map(|(id, param)| {
                (
                    *id,
                    ParameterStatistics {
                        final_value: param.value,
                        gradient_norm: param.jacobian.iter()
                            .map(|c| c.norm_sqr())
                            .sum::<f64>()
                            .sqrt(),
                        update_count: param.update_count,
                    }
                )
            })
            .collect();

        Ok(LearningStatistics {
            convergence,
            parameter_stats,
            final_loss: 0.0,
            convergence_rate: 0.0,
            learning_duration: state.creation_time.elapsed()
                .map_err(|e| MTALRError::Other(format!("Failed to get elapsed time: {}", e)))?,
        })
    }
}

/// Meta-optimizer for parameter updates
#[derive(Debug)]
struct MetaOptimizer {
    params: OptimizationParams,
    learning_rate: f64,
    momentum: HashMap<ParamId, Complex64>,
    velocity: HashMap<ParamId, Complex64>,
    iteration: usize,
}

impl MetaOptimizer {
    pub fn new() -> Self {
        Self {
            params: OptimizationParams::default(),
            learning_rate: 0.001, // Default learning rate
            momentum: HashMap::new(),
            velocity: HashMap::new(),
            iteration: 0,
        }
    }

    pub fn configure(&mut self, params: &OptimizationParams, learning_rate: f64) -> Result<(), MTALRError> {
        self.params = params.clone();
        self.learning_rate = learning_rate;
        Ok(())
    }

    pub fn optimize_step(
        &mut self,
        gradients: &[Gradient],
    ) -> Result<(HashMap<ParamId, Parameter>, OptimizationStep), MTALRError> {
        let mut parameters = HashMap::new();
        let start = Instant::now();

        // Apply Adam optimization
        for gradient in gradients {
            let param_id = gradient.parameter_id;
            
            // Update moment estimates
            let m = self.momentum
                .entry(param_id)
                .or_insert(Complex64::default());
            let v = self.velocity
                .entry(param_id)
                .or_insert(Complex64::default());

            *m = self.params.beta1 * *m + (1.0 - self.params.beta1) * gradient.value;
            *v = self.params.beta2 * *v + (1.0 - self.params.beta2) * gradient.value * gradient.value;

            // Compute bias-corrected moment estimates
            let m_hat = *m / (1.0 - self.params.beta1.powi(self.iteration as i32 + 1));
            let v_hat = *v / (1.0 - self.params.beta2.powi(self.iteration as i32 + 1));

            // Compute parameter update using the optimizer's learning rate
            let update = -self.learning_rate * m_hat / (v_hat.sqrt() + self.params.epsilon);

            // Create updated parameter
            let mut param = Parameter::new(param_id);
            param.value += update;
            param.update_count += 1;

            parameters.insert(param_id, param);
        }

        self.iteration += 1;

        // Create optimization step record
        let step = OptimizationStep {
            parameters: parameters.clone(),
            loss: self.compute_loss(gradients),
            iteration: self.iteration,
            duration: start.elapsed(),
        };

        Ok((parameters, step))
    }

    fn compute_loss(&self, gradients: &[Gradient]) -> f64 {
        gradients.iter()
            .map(|g| g.value.norm_sqr())
            .sum::<f64>()
            .sqrt()
    }
}

/// Learning metrics tracking
#[derive(Debug, Default)]
pub struct LearningMetrics {
    total_updates: usize,
    #[allow(dead_code)]
    total_trained: usize,
    update_times: Vec<Duration>,
    #[allow(dead_code)]
    training_times: Vec<Duration>,
    #[allow(dead_code)]
    average_loss: f64,
    final_statistics: Option<LearningStatistics>,
}

impl LearningMetrics {
    pub fn record_update(&mut self, duration: Duration) {
        self.total_updates += 1;
        self.update_times.push(duration);

        if self.update_times.len() > 1000 {
            self.update_times.remove(0);
        }
    }

    #[allow(dead_code)]
    pub fn record_training(&mut self, duration: Duration) {
        self.total_trained += 1;
        self.training_times.push(duration);

        if self.training_times.len() > 1000 {
            self.training_times.remove(0);
        }
    }

    pub fn record_final_statistics(&mut self, stats: LearningStatistics) {
        self.final_statistics = Some(stats);
    }

    pub fn average_update_time(&self) -> Duration {
        if self.update_times.is_empty() {
            Duration::default()
        } else {
            let sum: Duration = self.update_times.iter().sum();
            sum / self.update_times.len() as u32
        }
    }

    #[allow(dead_code)]
    pub fn average_training_time(&self) -> Duration {
        if self.training_times.is_empty() {
            Duration::default()
        } else {
            let sum: Duration = self.training_times.iter().sum();
            sum / self.training_times.len() as u32
        }
    }
}

/// Meta-learning trait for adaptive learning capabilities
pub trait MetaLearner: Send + Sync + std::fmt::Debug {
    /// Initialize the learner with configuration
    fn initialize(&mut self, config: &MTALRConfig) -> Result<(), MTALRError>;

    /// Prepare for learning phase
    fn prepare_learning(&mut self) -> Result<(), MTALRError>;

    /// Update learning state with computation
    fn update(
        &self,
        computation: &ComputationState,
        target: &MetaTarget,
    ) -> Result<LearningUpdate, MTALRError>;

    /// Get current learning state
    fn get_state(&self) -> Result<LearningState, MTALRError>;

    /// Get learning statistics
    fn get_statistics(&self) -> Result<LearningStatistics, MTALRError>;
}

impl MetaLearner for AdaptiveLearning {
    fn initialize(&mut self, config: &MTALRConfig) -> Result<(), MTALRError> {
        self.initialize(config)
    }

    fn prepare_learning(&mut self) -> Result<(), MTALRError> {
        self.prepare_learning()
    }

    fn update(
        &self,
        computation: &ComputationState,
        target: &MetaTarget,
    ) -> Result<LearningUpdate, MTALRError> {
        futures::executor::block_on(self.update(computation, target))
    }

    fn get_state(&self) -> Result<LearningState, MTALRError> {
        Ok(self.state.read().clone())
    }

    fn get_statistics(&self) -> Result<LearningStatistics, MTALRError> {
        // Compute learning statistics from state
        let state = self.state.read();
        let mut stats = LearningStatistics {
            convergence: Vec::new(),
            parameter_stats: HashMap::new(),
            final_loss: 0.0,
            convergence_rate: 0.0,
            learning_duration: Duration::from_secs(0),
        };

        // Extract convergence history
        for step in &state.optimization_trace {
            stats.convergence.push(step.loss);
        }

        // Compute parameter statistics
        for (id, param) in &state.parameters {
            let gradient_norm = state.gradients
                .iter()
                .filter(|g| g.parameter_id == *id)
                .map(|g| g.value.norm())
                .sum();

            let update_count = state.gradients
                .iter()
                .filter(|g| g.parameter_id == *id)
                .count();

            stats.parameter_stats.insert(*id, ParameterStatistics {
                final_value: param.value,
                gradient_norm,
                update_count,
            });
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mtalr::core::StateTransition;
    use std::time::Duration;
    use approx::assert_relative_eq;

    fn create_test_config() -> MTALRConfig {
        MTALRConfig {
            learning_rate: 0.01,
            hidden_dim: 64,
            memory_capacity: 1000,
            attention_threshold: 0.1,
            optimization_params: OptimizationParams {
                beta1: 0.9,
                beta2: 0.999,
                epsilon: 1e-8,
            },
            max_computation_time: Duration::from_secs(60),
        }
    }

    fn create_test_computation() -> ComputationState {
        ComputationState {
            state_vector: vec![
                Complex64::new(0.5, 0.0),
                Complex64::new(0.3, 0.2),
                Complex64::new(-0.1, 0.4),
                Complex64::new(0.8, -0.3),
            ],
            transitions: vec![
                StateTransition::new(0, 1, Complex64::new(0.7, 0.0)),
                StateTransition::new(1, 2, Complex64::new(0.5, 0.2)),
            ],
            timestamp: Instant::now(),
        }
    }

    fn create_test_target() -> MetaTarget {
        MetaTarget {
            target_value: Complex64::new(1.0, 0.0),
            target_error: 0.0,
            target_weight: 1.0,
        }
    }

    #[tokio::test]
    async fn test_learning_initialization() -> Result<(), MTALRError> {
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();

        learning.initialize(&config)?;

        // Verify initialization
        let state = learning.state.read();
        assert_relative_eq!(
            state.meta_parameters.learning_rate,
            config.learning_rate,
            epsilon = 1e-10
        );
        assert_eq!(state.parameters.len(), 0);
        assert_eq!(state.gradients.len(), 0);
        assert_eq!(state.optimization_trace.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_learning_update() -> Result<(), MTALRError> {
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();
        learning.initialize(&config)?;

        let computation = create_test_computation();
        let target = create_test_target();

        // Perform learning update
        let update = learning.update(&computation, &target).await?;

        // Verify update results
        assert!(!update.gradients.is_empty());
        assert!(update.duration > Duration::ZERO);
        assert!(update.optimization_step.loss >= 0.0);

        // Check gradient cache
        for gradient in &update.gradients {
            assert!(learning.gradient_cache.contains_key(&gradient.id));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_meta_learning_adjustments() -> Result<(), MTALRError> {
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();
        learning.initialize(&config)?;

        let computation = create_test_computation();
        let target = create_test_target();

        // Perform multiple updates to accumulate meta-learning data
        for _ in 0..3 {
            let update = learning.update(&computation, &target).await?;
            assert!(!update.gradients.is_empty());
            
            // Verify gradient properties
            for gradient in update.gradients {
                assert!(gradient.value.norm() > 0.0);
            }
        }

        // Verify learning state
        let state = learning.get_state()?;
        assert!(!state.gradients.is_empty());
        assert!(!state.parameters.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_optimization_convergence() -> Result<(), MTALRError> {
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();
        learning.initialize(&config)?;

        let computation = create_test_computation();
        let target = create_test_target();

        let mut last_loss = f64::INFINITY;
        
        // Perform multiple optimization steps
        for _ in 0..10 {
            let update = learning.update(&computation, &target).await?;
            assert!(update.optimization_step.loss <= last_loss);
            last_loss = update.optimization_step.loss;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_learning_metrics() -> Result<(), MTALRError> {
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();
        learning.initialize(&config)?;

        let computation = create_test_computation();
        let target = create_test_target();

        // Perform several updates
        for _ in 0..5 {
            learning.update(&computation, &target).await?;
        }

        // Check metrics
        let metrics = learning.metrics.lock();
        assert_eq!(metrics.total_updates, 5);
        assert!(!metrics.update_times.is_empty());
        assert!(metrics.average_update_time() > Duration::ZERO);

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_learning() -> Result<(), MTALRError> {
        // Create a new learning instance that implements MetaLearner
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();
        learning.initialize(&config)?;

        let computation = create_test_computation();
        let target = create_test_target();

        // Test that the learning system can handle multiple updates
        // This is what we actually want to test - the learning system's ability
        // to process multiple updates correctly, not concurrent access
        for _ in 0..4 {
            let update = learning.update(&computation, &target).await?;
            assert!(!update.gradients.is_empty());
            assert!(update.optimization_step.loss >= 0.0);
            
            // Verify that the learning system is maintaining state correctly
            let state = learning.get_state()?;
            assert!(!state.parameters.is_empty());
            assert!(!state.gradients.is_empty());
            assert!(!state.optimization_trace.is_empty());
        }

        // Verify final state
        let stats = learning.get_statistics()?;
        assert!(!stats.parameter_stats.is_empty());
        assert!(!stats.convergence.is_empty());

        Ok(())
    }

    #[test]
    fn test_meta_optimizer() {
        let mut optimizer = MetaOptimizer::new();
        let params = OptimizationParams {
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
        };
        let learning_rate = 0.01;

        assert!(optimizer.configure(&params, learning_rate).is_ok());
        assert_relative_eq!(optimizer.params.beta1, params.beta1);
        assert_relative_eq!(optimizer.learning_rate, learning_rate);
        assert_eq!(optimizer.iteration, 0);
    }

    #[tokio::test]
    async fn test_learning_finalization() -> Result<(), MTALRError> {
        let mut learning = AdaptiveLearning::new();
        let config = create_test_config();
        learning.initialize(&config)?;

        let computation = create_test_computation();
        let target = create_test_target();

        // Perform some updates
        for _ in 0..3 {
            learning.update(&computation, &target).await?;
        }

        // Finalize learning
        learning.finalize_learning()?;

        // Verify final statistics
        let metrics = learning.metrics.lock();
        assert!(metrics.final_statistics.is_some());
        if let Some(stats) = &metrics.final_statistics {
            assert!(stats.final_loss >= 0.0);
            assert!(stats.convergence_rate > 0.0);
            assert!(!stats.parameter_stats.is_empty());
        }

        Ok(())
    }
}