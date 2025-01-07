// src/mtalr/reasoning/adaptive.rs

use ndarray::{Array1, Array2};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, instrument};

use crate::mtalr::core::ComputationState;
use super::state::StateManager;

/// Adaptive reasoning errors
#[derive(Error, Debug)]
pub enum AdaptiveError {
    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("Processing error: {0}")]
    ProcessError(String),

    #[error("Memory error: {0}")]
    MemoryError(String),

    #[error("State error: {0}")]
    StateError(#[from] super::state::StateError),
}

/// Memory entry for adaptive reasoning
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    /// Input pattern
    pattern: Array1<Complex64>,
    /// Associated reasoning
    reasoning: Array1<Complex64>,
    /// Confidence score
    confidence: f64,
}

/// Configuration for adaptive reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveConfig {
    /// Hidden state dimension
    pub hidden_dim: usize,
    /// Memory capacity
    pub memory_capacity: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Attention threshold
    pub attention_threshold: f64,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            hidden_dim: 64,
            memory_capacity: 1000,
            learning_rate: 0.01,
            attention_threshold: 0.1,
        }
    }
}

/// Adaptive reasoning implementation
#[derive(Debug)]
pub struct AdaptiveReasoning {
    /// Configuration
    config: AdaptiveConfig,
    /// State manager
    state_manager: StateManager,
    /// Memory entries
    memory: Vec<MemoryEntry>,
    /// Pattern similarity matrix
    similarity: Array2<f64>,
}

impl AdaptiveReasoning {
    /// Create new adaptive reasoning
    #[instrument(skip(config))]
    pub fn new(config: AdaptiveConfig) -> Result<Self, AdaptiveError> {
        let state_manager = StateManager::new(config.hidden_dim)
            .map_err(|e| AdaptiveError::InitError(e.to_string()))?;

        let similarity = Array2::zeros((0, 0));

        debug!("Initialized adaptive reasoning");

        Ok(Self {
            config,
            state_manager,
            memory: Vec::new(),
            similarity,
        })
    }

    /// Process input and generate reasoning
    #[instrument(skip(self, input))]
    pub fn process(
        &mut self,
        input: &Array1<Complex64>,
    ) -> Result<Array1<Complex64>, AdaptiveError> {
        // Update hidden state
        let hidden = self.update_hidden_state(input)?;

        // Find relevant memories and compute attention - do all immutable operations first
        let (memories, attention) = self.find_relevant_memories(&hidden)?;
        let reasoning = self.generate_reasoning(&memories, &hidden)?;

        // Now do the mutable operations
        self.state_manager.update_attention(Some(attention))?;

        // Update memory with new entry if confidence is high enough
        if self.compute_confidence(&reasoning) > self.config.attention_threshold {
            self.update_memory(input.clone(), reasoning.clone())?;
        }

        Ok(reasoning)
    }

    /// Update hidden state
    fn update_hidden_state(
        &mut self,
        input: &Array1<Complex64>,
    ) -> Result<Array1<Complex64>, AdaptiveError> {
        // Get a copy of the hidden state first
        let hidden = self.state_manager.current_state().hidden().to_owned();
        
        // Now we can mutably borrow state_manager
        let update = self.state_manager.compute_update(input, &hidden)?;
        Ok(update)
    }

    /// Find relevant memories based on hidden state
    fn find_relevant_memories(
        &self,
        hidden: &Array1<Complex64>,
    ) -> Result<(Vec<&MemoryEntry>, Array1<f64>), AdaptiveError> {
        if self.memory.is_empty() {
            return Ok((Vec::new(), Array1::zeros(0)));
        }

        // Compute similarity scores
        let mut scores: Vec<f64> = self.memory
            .iter()
            .map(|entry| {
                let similarity = hidden
                    .iter()
                    .zip(entry.pattern.iter())
                    .map(|(h, p)| (h * p.conj()).norm())
                    .sum::<f64>();
                similarity / (hidden.len() as f64)
            })
            .collect();

        // Normalize scores to get attention weights
        let total: f64 = scores.iter().sum();
        if total > 0.0 {
            scores.iter_mut().for_each(|s| *s /= total);
        }

        // Select memories above threshold
        let attention = Array1::from_vec(scores.clone());
        let relevant: Vec<&MemoryEntry> = self.memory
            .iter()
            .zip(scores.iter())
            .filter(|(_, &score)| score > self.config.attention_threshold)
            .map(|(entry, _)| entry)
            .collect();

        Ok((relevant, attention))
    }

    /// Generate reasoning based on memories and current state
    fn generate_reasoning(
        &self,
        memories: &[&MemoryEntry],
        hidden: &Array1<Complex64>,
    ) -> Result<Array1<Complex64>, AdaptiveError> {
        if memories.is_empty() {
            // If no relevant memories, use transformed hidden state
            return Ok(hidden.clone());
        }

        // Combine memories weighted by similarity
        let mut combined = Array1::zeros(self.config.hidden_dim);
        for memory in memories {
            let similarity = hidden
                .iter()
                .zip(memory.pattern.iter())
                .map(|(h, p)| (h * p.conj()).norm())
                .sum::<f64>();
            let weight = similarity / (hidden.len() as f64);
            combined += &(memory.reasoning.mapv(|x| x * weight));
        }

        // Mix with current hidden state
        let alpha = 0.7; // Balance between memory and current state
        let reasoning = &combined * alpha + hidden * (1.0 - alpha);

        Ok(reasoning)
    }

    /// Compute confidence score for reasoning
    fn compute_confidence(&self, reasoning: &Array1<Complex64>) -> f64 {
        // Use norm of reasoning vector as confidence measure
        let norm = reasoning.iter().map(|x| x.norm()).sum::<f64>();
        norm / (reasoning.len() as f64)
    }

    /// Update memory with new entry
    fn update_memory(
        &mut self,
        pattern: Array1<Complex64>,
        reasoning: Array1<Complex64>,
    ) -> Result<(), AdaptiveError> {
        let confidence = self.compute_confidence(&reasoning);

        // Add new entry
        let entry = MemoryEntry {
            pattern,
            reasoning,
            confidence,
        };

        // Maintain memory capacity
        if self.memory.len() >= self.config.memory_capacity {
            // Remove entry with lowest confidence
            if let Some(min_idx) = self.memory
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.confidence.partial_cmp(&b.confidence).unwrap())
                .map(|(i, _)| i)
            {
                self.memory.remove(min_idx);
            }
        }

        self.memory.push(entry);
        self.update_similarity_matrix()?;

        Ok(())
    }

    /// Update similarity matrix after memory changes
    fn update_similarity_matrix(&mut self) -> Result<(), AdaptiveError> {
        let n = self.memory.len();
        let mut similarity = Array2::zeros((n, n));

        for i in 0..n {
            for j in 0..n {
                let sim = self.memory[i]
                    .pattern
                    .iter()
                    .zip(self.memory[j].pattern.iter())
                    .map(|(a, b)| (a * b.conj()).norm())
                    .sum::<f64>();
                similarity[[i, j]] = sim / (self.config.hidden_dim as f64);
            }
        }

        self.similarity = similarity;
        Ok(())
    }

    /// Reset state and memory
    pub fn reset(&mut self) -> Result<(), AdaptiveError> {
        self.state_manager.reset()?;
        self.memory.clear();
        self.similarity = Array2::zeros((0, 0));
        Ok(())
    }

    /// Adapt based on feedback
    pub fn adapt(&mut self, learning_rate: Option<f64>) -> Result<(), AdaptiveError> {
        let lr = learning_rate.unwrap_or(self.config.learning_rate);
        self.state_manager.update_dynamics(lr)?;
        Ok(())
    }
}

/// Trait for adaptive reasoning capabilities
pub trait AdaptiveReasoner: Send + Sync + std::fmt::Debug {
    /// Process input and generate reasoning
    fn process(&mut self, input: &Array1<Complex64>) -> Result<Array1<Complex64>, AdaptiveError>;

    /// Process computation state
    fn process_state(&mut self, state: &ComputationState) -> Result<(), AdaptiveError>;

    /// Reset the reasoner state
    fn reset(&mut self) -> Result<(), AdaptiveError>;

    /// Adapt the reasoner parameters
    fn adapt(&mut self, learning_rate: Option<f64>) -> Result<(), AdaptiveError>;
}

impl AdaptiveReasoner for AdaptiveReasoning {
    fn process(&mut self, input: &Array1<Complex64>) -> Result<Array1<Complex64>, AdaptiveError> {
        self.process(input)
    }

    fn process_state(&mut self, state: &ComputationState) -> Result<(), AdaptiveError> {
        // Convert state vector to input array
        let input = Array1::from_vec(state.state_vector.clone());
        
        // Process the state vector
        self.process(&input)?;
        
        Ok(())
    }

    fn reset(&mut self) -> Result<(), AdaptiveError> {
        self.reset()
    }

    fn adapt(&mut self, learning_rate: Option<f64>) -> Result<(), AdaptiveError> {
        self.adapt(learning_rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_input() -> Array1<Complex64> {
        Array1::from_vec(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 1.0),
            Complex64::new(-1.0, -1.0),
        ])
    }

    #[test]
    fn test_initialization() -> Result<(), AdaptiveError> {
        let config = AdaptiveConfig {
            hidden_dim: 3,
            memory_capacity: 10,
            learning_rate: 0.01,
            attention_threshold: 0.1,
        };
        let reasoning = AdaptiveReasoning::new(config)?;

        assert_eq!(reasoning.memory.len(), 0);
        assert_eq!(reasoning.similarity.shape(), &[0, 0]);

        Ok(())
    }

    #[test]
    fn test_process_input() -> Result<(), AdaptiveError> {
        let config = AdaptiveConfig {
            hidden_dim: 3,
            memory_capacity: 10,
            learning_rate: 0.01,
            attention_threshold: 0.1,
        };
        let mut reasoning = AdaptiveReasoning::new(config)?;

        let input = create_test_input();
        let result = reasoning.process(&input)?;

        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|x| x.norm() > 0.0));

        Ok(())
    }

    #[test]
    fn test_memory_capacity() -> Result<(), AdaptiveError> {
        let config = AdaptiveConfig {
            hidden_dim: 3,
            memory_capacity: 2,
            learning_rate: 0.01,
            attention_threshold: 0.0, // Set low to ensure entries are added
        };
        let mut reasoning = AdaptiveReasoning::new(config)?;

        // Add three entries
        for _ in 0..3 {
            let input = create_test_input();
            reasoning.process(&input)?;
        }

        assert_eq!(reasoning.memory.len(), 2);
        assert_eq!(reasoning.similarity.shape(), &[2, 2]);

        Ok(())
    }

    #[test]
    fn test_adaptation() -> Result<(), AdaptiveError> {
        let config = AdaptiveConfig {
            hidden_dim: 3,
            memory_capacity: 10,
            learning_rate: 0.1,
            attention_threshold: 0.1,
        };
        let mut reasoning = AdaptiveReasoning::new(config)?;

        reasoning.adapt(None)?;
        reasoning.adapt(Some(0.2))?;

        Ok(())
    }

    #[test]
    fn test_reset() -> Result<(), AdaptiveError> {
        let config = AdaptiveConfig {
            hidden_dim: 3,
            memory_capacity: 10,
            learning_rate: 0.01,
            attention_threshold: 0.0,
        };
        let mut reasoning = AdaptiveReasoning::new(config)?;

        // Add some entries
        let input = create_test_input();
        reasoning.process(&input)?;

        // Reset
        reasoning.reset()?;

        assert_eq!(reasoning.memory.len(), 0);
        assert_eq!(reasoning.similarity.shape(), &[0, 0]);

        Ok(())
    }
}
