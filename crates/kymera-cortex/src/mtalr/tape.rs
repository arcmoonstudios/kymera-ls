use std::{
    collections::{VecDeque, HashMap},
    sync::{Arc, atomic::{AtomicUsize, Ordering}},
    time::{Duration, SystemTime},
    num::NonZeroUsize,
};
use crate::err::{CortexError, TapeError, Result, Context};
use num_complex::Complex64;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn, debug};

/// Strongly typed position for tape operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(NonZeroUsize);

impl Position {
    pub fn new(pos: usize) -> Option<Self> {
        NonZeroUsize::new(pos.saturating_add(1)).map(Self)
    }

    pub fn get(&self) -> usize {
        self.0.get().saturating_sub(1)
    }
}

/// Strongly typed symbol value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolValue(u64);

impl SymbolValue {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

/// Tape symbol with quantum properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TapeSymbol {
    value: SymbolValue,
    amplitude: Complex64,
    creation_time: u64, // milliseconds since Unix epoch
}

impl TapeSymbol {
    pub fn new(value: u64) -> Result<Self> {
        let creation_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_millis() as u64;
            
        Ok(Self {
            value: SymbolValue::new(value),
            amplitude: Complex64::new(1.0, 0.0),
            creation_time,
        })
    }

    pub fn with_amplitude(value: u64, amplitude: Complex64) -> Result<Self> {
        if (amplitude.norm_sqr() - 1.0).abs() > 1e-6 {
            return Err(CortexError::Tape(TapeError::QuantumError("Amplitude not normalized".into())))
                .context("Invalid quantum amplitude");
        }

        let creation_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_millis() as u64;

        Ok(Self {
            value: SymbolValue::new(value),
            amplitude,
            creation_time,
        })
    }

    pub fn as_index(&self) -> usize {
        self.value.get() as usize
    }

    pub fn age(&self) -> Result<Duration> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_millis() as u64;
        Ok(Duration::from_millis(now - self.creation_time))
    }

    pub fn quantum_state(&self) -> Complex64 {
        self.amplitude
    }
}

/// Builder for TuringTape initialization
#[derive(Default)]
pub struct TuringTapeBuilder {
    size: Option<NonZeroUsize>,
    symbols: Vec<TapeSymbol>,
    initial_head: Option<Position>,
}

impl TuringTapeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = NonZeroUsize::new(size);
        self
    }

    pub fn symbols(mut self, symbols: Vec<TapeSymbol>) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn initial_head(mut self, position: Position) -> Self {
        self.initial_head = Some(position);
        self
    }

    pub fn build(self) -> Result<TuringTape> {
        let size = self.size.ok_or_else(|| 
            TapeError::InvalidSymbol("Size must be non-zero".into()))?;

        let mut tape = TuringTape {
            symbols: VecDeque::with_capacity(size.get()),
            head_position: self.initial_head.map(|p| p.get()).unwrap_or(0),
            entanglement_map: Arc::new(RwLock::new(EntanglementMap::new())),
            stats: TapeStatistics::default(),
            cached_avg_time: Arc::new(RwLock::new(None)),
        };

        tape.initialize(size.get(), self.symbols)?;
        Ok(tape)
    }
}

/// Quantum-enhanced Turing tape
#[derive(Debug)]
pub struct TuringTape {
    symbols: VecDeque<TapeSymbol>,
    head_position: usize,
    entanglement_map: Arc<RwLock<EntanglementMap>>,
    stats: TapeStatistics,
    cached_avg_time: Arc<RwLock<Option<Duration>>>,
}

impl TuringTape {
    pub fn builder() -> TuringTapeBuilder {
        TuringTapeBuilder::new()
    }

    /// Initialize tape with size and symbols
    #[instrument]
    fn initialize(
        &mut self,
        size: usize,
        symbol_set: Vec<TapeSymbol>,
    ) -> Result<()> {
        info!("Initializing Turing tape with size {}", size);

        for _ in 0..size {
            self.symbols.push_back(TapeSymbol::new(0)?);
        }

        for (i, symbol) in symbol_set.into_iter().enumerate() {
            if i >= size {
                debug!("Symbol set larger than tape size, truncating");
                break;
            }
            self.symbols[i] = symbol;
        }

        let mut entanglement = self.entanglement_map.write();
        entanglement.initialize(size)
            .map_err(|e| TapeError::QuantumError(e.to_string()))?;

        Ok(())
    }

    /// Read symbol at current head position
    pub fn read_symbol(&self) -> Result<TapeSymbol> {
        self.read_symbol_at(self.head_position)
    }

    /// Read symbol at specific position
    pub fn read_symbol_at(&self, position: usize) -> Result<TapeSymbol> {
        if position >= self.symbols.len() {
            return Err(CortexError::Tape(TapeError::OutOfBounds(format!("Position {} out of bounds", position))))
                .context("Failed to read symbol");
        }

        let symbol = self.symbols[position].clone();
        self.stats.record_read(&symbol);
        Ok(symbol)
    }

    /// Write symbol at current head position
    pub fn write_symbol(&mut self, symbol: TapeSymbol) -> Result<()> {
        self.write_symbol_at(self.head_position, symbol)
    }

    /// Write symbol at specific position
    pub fn write_symbol_at(
        &mut self,
        position: usize,
        symbol: TapeSymbol,
    ) -> Result<()> {
        if position >= self.symbols.len() {
            return Err(CortexError::Tape(TapeError::OutOfBounds(format!("Position {} out of bounds", position))))
                .context("Failed to write symbol");
        }

        let mut entanglement = self.entanglement_map.write();
        entanglement.update_symbol(position, &symbol)
            .map_err(CortexError::Tape)
            .context("Failed to update entanglement")?;

        self.symbols[position] = symbol.clone();
        let _ = self.stats.record_write(&symbol);
        
        // Invalidate cache
        *self.cached_avg_time.write() = None;
        
        Ok(())
    }

    /// Move tape head
    pub fn move_head(&mut self, direction: Direction) -> Result<()> {
        match direction {
            Direction::Left => {
                if self.head_position == 0 {
                    return Err(CortexError::Tape(TapeError::OutOfBounds("Cannot move left from position 0".into())))
                        .context("Failed to move head left");
                }
                self.head_position -= 1;
            }
            Direction::Right => {
                if self.head_position >= self.symbols.len() - 1 {
                    return Err(CortexError::Tape(TapeError::OutOfBounds("Cannot move right from end of tape".into())))
                        .context("Failed to move head right");
                }
                self.head_position += 1;
            }
        }
        Ok(())
    }

    /// Get current head position
    pub fn head_position(&self) -> Position {
        Position::new(self.head_position).expect("Head position is always valid")
    }

    /// Get all symbols on tape
    pub fn get_symbols(&self) -> Vec<TapeSymbol> {
        let mut symbols = Vec::with_capacity(self.symbols.len());
        symbols.extend(self.symbols.iter().cloned());
        symbols
    }

    /// Get tape statistics
    pub fn statistics(&self) -> &TapeStatistics {
        &self.stats
    }

    /// Check quantum coherence of tape region
    pub fn check_coherence(&self, start: usize, end: usize) -> Result<bool> {
        if start >= self.symbols.len() || end >= self.symbols.len() {
            return Err(CortexError::Tape(TapeError::OutOfBounds(format!("Range {}..{} out of bounds", start, end))))
                .context("Failed to check coherence");
        }

        let entanglement = self.entanglement_map.read();
        entanglement.check_coherence(start..=end)
            .map_err(CortexError::Tape)
            .context("Failed to check quantum coherence")
    }
}

/// Tape movement direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// Quantum entanglement tracking
#[derive(Debug, Serialize, Deserialize)]
struct EntanglementMap {
    entanglements: Vec<Vec<usize>>,
    coherence_times: Vec<u64>,
}

impl EntanglementMap {
    pub fn new() -> Self {
        Self {
            entanglements: Vec::new(),
            coherence_times: Vec::new(),
        }
    }

    pub fn initialize(&mut self, size: usize) -> std::result::Result<(), TapeError> {
        self.entanglements = vec![Vec::new(); size];
        self.coherence_times = vec![0; size];
        Ok(())
    }

    pub fn update_symbol(
        &mut self,
        position: usize,
        symbol: &TapeSymbol,
    ) -> std::result::Result<(), TapeError> {
        if position >= self.coherence_times.len() {
            return Err(TapeError::OutOfBounds(format!("Position {} out of bounds", position)));
        }

        self.coherence_times[position] = symbol.creation_time;

        let amplitude = symbol.quantum_state();
        if amplitude.norm() > 1e-6 {
            let entangled_positions: Vec<_> = self.entanglements
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != position && self.is_entangled(amplitude, i))
                .map(|(i, _)| i)
                .collect();

            for i in entangled_positions {
                if !self.entanglements[position].contains(&i) {
                    self.entanglements[position].push(i);
                }
                if !self.entanglements[i].contains(&position) {
                    self.entanglements[i].push(position);
                }
            }
        }
        Ok(())
    }

    pub fn check_coherence(&self, range: std::ops::RangeInclusive<usize>) -> std::result::Result<bool, TapeError> {
        for pos in range {
            if pos >= self.coherence_times.len() {
                return Err(TapeError::OutOfBounds(format!("Position {} out of bounds", pos)));
            }

            if self.coherence_times[pos] == 0 {
                return Ok(false);
            }

            for &entangled_pos in &self.entanglements[pos] {
                if entangled_pos >= self.coherence_times.len() {
                    return Err(TapeError::OutOfBounds(format!("Entangled position {} out of bounds", entangled_pos)));
                }
                if self.coherence_times[entangled_pos] == 0 {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Check if a quantum state at the given position is entangled based on its amplitude
    /// and the quantum decoherence threshold.
    ///
    /// The entanglement is determined by:
    /// 1. The amplitude's squared norm (probability density) being above 0.5
    /// 2. The position's coherence time being within acceptable bounds
    /// 3. The presence of other entangled states in the quantum system
    ///
    /// # Arguments
    /// * `amplitude` - The complex amplitude of the quantum state
    /// * `position` - The position to check for entanglement
    ///
    /// # Returns
    /// `true` if the state is entangled, `false` otherwise
    fn is_entangled(&self, amplitude: Complex64, position: usize) -> bool {
        // Basic probability density check
        if amplitude.norm_sqr() <= 0.5 {
            return false;
        }

        // Check if the position has valid coherence time
        if position >= self.coherence_times.len() || self.coherence_times[position] == 0 {
            return false;
        }

        // Check for existing entanglements
        !self.entanglements[position].is_empty()
    }
}

/// Tape operation statistics with thread-safe counters
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TapeStatistics {
    #[serde(skip)]
    total_reads: Arc<AtomicUsize>,
    #[serde(skip)]
    total_writes: Arc<AtomicUsize>,
    #[serde(skip)]
    symbol_frequencies: Arc<RwLock<HashMap<u64, usize>>>,
    #[serde(skip)]
    operation_times: Arc<RwLock<VecDeque<Duration>>>,
    #[serde(skip)]
    cached_avg_time: Arc<RwLock<Option<Duration>>>,
}

impl TapeStatistics {
    pub fn record_read(&self, symbol: &TapeSymbol) {
        self.total_reads.fetch_add(1, Ordering::SeqCst);
        let mut frequencies = self.symbol_frequencies.write();
        *frequencies.entry(symbol.value.get()).or_insert(0) += 1;
    }

    pub fn record_write(&self, symbol: &TapeSymbol) -> Result<()> {
        self.total_writes.fetch_add(1, Ordering::SeqCst);
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| TapeError::TimeError(e.to_string()))?;
        let symbol_time = Duration::from_millis(symbol.creation_time as u64);
        let mut times = self.operation_times.write();
        times.push_back(now - symbol_time);

        if times.len() > 1000 {
            times.pop_front();
        }
        Ok(())
    }

    pub fn average_operation_time(&self) -> Duration {
        if let Some(cached) = *self.cached_avg_time.read() {
            return cached;
        }

        let times = self.operation_times.read();
        let avg = if times.is_empty() {
            Duration::default()
        } else {
            let sum: Duration = times.iter().sum();
            Duration::from_millis((sum.as_millis() / times.len() as u128) as u64)
        };

        *self.cached_avg_time.write() = Some(avg);
        avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use test_case::test_case;
    use proptest::prelude::*;

    #[test]
    fn test_tape_initialization() {
        let symbols = vec![
            TapeSymbol::new(1).unwrap(),
            TapeSymbol::new(2).unwrap(),
            TapeSymbol::new(3).unwrap(),
        ];
        let tape = TuringTape::builder()
            .size(10)
            .symbols(symbols)
            .build()
            .unwrap();
        assert_eq!(tape.symbols.len(), 10);
        assert_eq!(tape.head_position, 0);
    }

    #[test]
    fn test_symbol_operations() {
        let tape = TuringTape::builder()
            .size(10)
            .symbols(vec![])
            .build()
            .unwrap();

        let symbol = TapeSymbol::with_amplitude(42, Complex64::new(1.0, 0.0)).unwrap();
        let mut tape = tape;
        assert!(tape.write_symbol(symbol.clone()).is_ok());
        let read = tape.read_symbol().unwrap();
        assert_eq!(read.value, symbol.value);
        assert_eq!(read.amplitude, symbol.amplitude);
    }

    #[test]
    fn test_head_movement() {
        let mut tape = TuringTape::builder()
            .size(10)
            .symbols(vec![])
            .build()
            .unwrap();

        assert!(tape.move_head(Direction::Right).is_ok());
        assert_eq!(tape.head_position().get(), 1);
        assert!(tape.move_head(Direction::Left).is_ok());
        assert_eq!(tape.head_position().get(), 0);
        assert!(tape.move_head(Direction::Left).is_err());
    }

    #[test]
    fn test_quantum_coherence() {
        let mut tape = TuringTape::builder()
            .size(10)
            .symbols(vec![])
            .build()
            .unwrap();

        let symbol1 = TapeSymbol::with_amplitude(1, Complex64::new(1.0, 0.0)).unwrap();
        let symbol2 = TapeSymbol::with_amplitude(2, Complex64::new(0.0, 1.0)).unwrap();

        tape.write_symbol_at(0, symbol1).unwrap();
        tape.write_symbol_at(1, symbol2).unwrap();

        assert!(tape.check_coherence(0, 1).unwrap());

        std::thread::sleep(Duration::from_millis(150));
        assert!(!tape.check_coherence(0, 1).unwrap());
    }

    #[test_case(0, Direction::Left; "left bound error")]
    #[test_case(9, Direction::Right; "right bound error")]
    #[test_case(5, Direction::Left; "valid left move")]
    #[test_case(5, Direction::Right; "valid right move")]
    fn test_head_movement_bounds(start_pos: usize, direction: Direction) {
        let mut tape = TuringTape::builder()
            .size(10)
            .symbols(vec![])
            .build()
            .unwrap();
        tape.head_position = start_pos;
        let result = tape.move_head(direction);
        match (start_pos, direction) {
            (0, Direction::Left) | (9, Direction::Right) => assert!(result.is_err()),
            _ => assert!(result.is_ok()),
        }
    }

    proptest! {
        #[test]
        fn test_symbol_quantum_properties(
            value in 0u64..100,
            re in -1.0..1.0f64,
            im in -1.0..1.0f64
        ) {
            let amplitude = Complex64::new(re, im);
            let symbol = TapeSymbol::with_amplitude(value, amplitude).unwrap();
            prop_assert_eq!(symbol.value.get(), value);
            prop_assert!((symbol.amplitude - amplitude).norm() < 1e-10);
        }
    }
}