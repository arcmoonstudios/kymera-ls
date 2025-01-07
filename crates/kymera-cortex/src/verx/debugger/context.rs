//! Provides the debugging context, including memory, scope, and event structures.

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, instrument};

use crate::{
    err::ContextError, Result as CortexResult,
};

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

/// Debug scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    /// Scope ID
    pub id: String,
    /// Parent scope ID
    pub parent_id: Option<String>,
    /// Scope name
    pub name: String,
    /// Scope start time
    #[serde(with = "instant_serde")]
    pub start_time: Instant,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            id: String::new(),
            parent_id: None,
            name: String::new(),
            start_time: Instant::now(),
        }
    }
}

/// Memory state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryState {
    /// Memory ID
    pub id: String,
    /// Memory data
    pub data: Vec<u8>,
    /// Memory timestamp
    #[serde(with = "instant_serde")]
    pub timestamp: Instant,
}

impl Default for MemoryState {
    fn default() -> Self {
        Self {
            id: String::new(),
            data: Vec::new(),
            timestamp: Instant::now(),
        }
    }
}

/// Debug event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// Event data
    pub data: Vec<u8>,
    /// Event timestamp
    #[serde(with = "instant_serde")]
    pub timestamp: Instant,
}

impl DebugEvent {
    /// Simple constructor if you want to pass in a quick event type/name.
    pub fn new_default(evt_type: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: evt_type.to_string(),
            data: vec![],
            timestamp: Instant::now(),
        }
    }
}

impl Default for DebugEvent {
    fn default() -> Self {
        Self::new_default("GenericDebugEvent")
    }
}

/// Scope metadata
#[derive(Debug, Clone, Default)]
pub struct ScopeMetadata {
    /// Whether scope has been analyzed
    pub analyzed: bool,
    /// Number of references
    pub reference_count: usize,
    /// Whether scope contains unsafe code
    pub contains_unsafe: bool,
    /// Whether scope has side effects
    pub has_side_effects: bool,
}

/// Debugger context configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    /// Context dimension
    pub context_dim: usize,
    /// Memory capacity
    pub memory_capacity: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Attention threshold
    pub attention_threshold: f64,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            context_dim: 32,
            memory_capacity: 100,
            learning_rate: 0.01,
            attention_threshold: 0.1,
        }
    }
}

/// Debug context entry
#[derive(Debug, Clone)]
pub struct ContextEntry {
    /// Input pattern
    #[allow(dead_code)]
    pattern: Array1<Complex64>,
    /// Context vector
    context: Array1<Complex64>,
    /// Timestamp
    #[allow(dead_code)]
    timestamp: u64,
}

/// Debugger context implementation
#[derive(Debug)]
pub struct DebuggerContext {
    /// Configuration
    config: ContextConfig,
    /// Context entries
    entries: Vec<ContextEntry>,
    /// Current context
    current_context: Array1<Complex64>,
    /// Context transition matrix
    transition: Array2<Complex64>,
    /// Global timestamp
    timestamp: u64,
}

/// Context-level errors are mapped to `ContextError` from `err/`.
impl DebuggerContext {
    /// Create new debugger context
    #[instrument(skip(config))]
    pub fn new(config: ContextConfig) -> CortexResult<Self> {
        let current_context = Array1::zeros(config.context_dim);

        // Initialize transition as identity + small random noise
        let mut transition = Array2::eye(config.context_dim);
        transition.mapv_inplace(|x| {
            x + Complex64::new(
                rand::random::<f64>() * 0.1 - 0.05,
                rand::random::<f64>() * 0.1 - 0.05,
            )
        });

        debug!("Initialized debugger context");

        Ok(Self {
            config,
            entries: Vec::new(),
            current_context,
            transition,
            timestamp: 0,
        })
    }

    /// Process input and update context
    #[instrument(skip(self, input))]
    pub fn process(&mut self, input: &Array1<Complex64>) -> CortexResult<Array1<Complex64>> {
        if input.len() != self.config.context_dim {
            return Err(ContextError::Update(format!(
                "Expected input dimension {}, got {}",
                self.config.context_dim,
                input.len()
            )).into());
        }

        let context = self.update_context(input)?;
        if self.is_significant(&context) {
            self.store_context(input.clone(), context.clone())?;
        }

        Ok(context)
    }

    /// Update context based on input
    fn update_context(&mut self, input: &Array1<Complex64>) -> CortexResult<Array1<Complex64>> {
        // Apply transition
        let state_contribution = self.transition.dot(&self.current_context);

        // Combine with input
        let mut context = &state_contribution + input;

        // Complex tanh
        context.mapv_inplace(|x| {
            let r = x.norm();
            let theta = x.arg();
            Complex64::from_polar(r.tanh(), theta)
        });

        self.current_context = context.clone();
        Ok(context)
    }

    fn is_significant(&self, context: &Array1<Complex64>) -> bool {
        let norm = context.iter().map(|x| x.norm()).sum::<f64>();
        let avg_norm = norm / (context.len() as f64);
        avg_norm > self.config.attention_threshold
    }

    fn store_context(
        &mut self,
        pattern: Array1<Complex64>,
        context: Array1<Complex64>,
    ) -> CortexResult<()> {
        let entry = ContextEntry {
            pattern,
            context,
            timestamp: self.timestamp,
        };

        if self.entries.len() >= self.config.memory_capacity {
            self.entries.remove(0);
        }

        self.entries.push(entry);
        self.timestamp += 1;
        Ok(())
    }

    /// Expose read-only history
    pub fn history(&self) -> &[ContextEntry] {
        &self.entries
    }

    /// Find similar contexts
    pub fn find_similar(
        &self,
        context: &Array1<Complex64>,
        threshold: Option<f64>,
    ) -> Vec<&ContextEntry> {
        let threshold = threshold.unwrap_or(self.config.attention_threshold);
        self.entries
            .iter()
            .filter(|entry| {
                let similarity = self.compute_similarity(context, &entry.context);
                similarity > threshold
            })
            .collect()
    }

    fn compute_similarity(&self, a: &Array1<Complex64>, b: &Array1<Complex64>) -> f64 {
        let dot_product = a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x * y.conj()).norm())
            .sum::<f64>();
        dot_product / (self.config.context_dim as f64)
    }

    /// Reset context
    pub fn reset(&mut self) -> CortexResult<()> {
        self.current_context.fill(Complex64::new(0.0, 0.0));
        self.entries.clear();
        self.timestamp = 0;
        Ok(())
    }

    /// Update transition dynamics
    pub fn update_dynamics(&mut self, learning_rate: Option<f64>) -> CortexResult<()> {
        let lr = learning_rate.unwrap_or(self.config.learning_rate);

        self.transition.mapv_inplace(|x| {
            x + Complex64::new(
                rand::random::<f64>() * lr - lr / 2.0,
                rand::random::<f64>() * lr - lr / 2.0,
            )
        });

        let norm = self.transition.iter().map(|x| x.norm()).sum::<f64>().sqrt();
        if norm > 0.0 {
            self.transition.mapv_inplace(|x| x / norm);
        }
        Ok(())
    }
}
