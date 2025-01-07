use std::sync::Arc;
use anyhow::{Context, Result as AnalyzerResult};

use crate::err::AnalysisError;
use crate::types::Type;

/// Represents the visibility of a symbol
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}

/// Represents the kind of a symbol
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Type,
    Variable,
    Parameter,
    Field,
}

/// Metadata for a symbol
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SymbolMetadata {
    pub reference_count: usize,
    pub has_side_effects: bool,
    pub is_constant: bool,
    pub is_deprecated: bool,
    pub deprecation_message: Option<String>,
}

/// Represents a symbol in the analysis phase
#[derive(Debug, Clone)]
pub struct AnalysisSymbol {
    /// The name of the symbol
    pub name: String,
    /// The kind of symbol
    pub kind: SymbolKind,
    /// The type of the symbol
    pub ty: Type,
    /// The scope level where the symbol is defined
    pub scope_level: usize,
    /// Documentation comments
    pub documentation: Option<String>,
    /// Additional metadata
    pub metadata: SymbolMetadata,
    /// Symbol visibility
    pub visibility: Visibility,
    /// Whether the symbol is mutable
    pub is_mutable: bool,
}

/// Statistics for symbol table operations
#[derive(Debug, Default)]
pub struct SymbolTableStats {
    pub total_symbols: usize,
    pub total_lookups: usize,
    pub successful_lookups: usize,
    pub failed_lookups: usize,
    pub unresolved_references: usize,
}

/// Symbol table for tracking symbols during analysis
#[derive(Debug)]
pub struct AnalysisTable {
    /// Stack of scopes, each containing a map of symbols
    scopes: Vec<Vec<Arc<AnalysisSymbol>>>,
    /// Statistics for symbol table operations
    stats: SymbolTableStats,
}

impl AnalysisTable {
    /// Creates a new symbol table
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
            stats: SymbolTableStats::default(),
        }
    }

    /// Gets the current scope level
    pub fn current_level(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Pushes a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    /// Pops the current scope from the stack
    pub fn pop_scope(&mut self) -> AnalyzerResult<()> {
        if self.scopes.len() <= 1 {
            return Err(AnalysisError::scope_error("Cannot pop global scope"))
                .context("Attempted to pop global scope");
        }
        self.scopes.pop();
        Ok(())
    }

    /// Defines a new symbol in the current scope
    pub fn define(&mut self, symbol: AnalysisSymbol) -> AnalyzerResult<()> {
        let current_scope = self.scopes.last_mut()
            .ok_or_else(|| AnalysisError::scope_error("No active scope"))
            .context("Failed to access current scope")?;
        
        // Check for duplicate definitions in the current scope
        if current_scope.iter().any(|s| s.name == symbol.name) {
            return Err(AnalysisError::symbol_error(format!(
                "Symbol {} already defined in current scope",
                symbol.name
            ))).context("Duplicate symbol definition");
        }
        
        self.stats.total_symbols += 1;
        current_scope.push(Arc::new(symbol));
        Ok(())
    }

    /// Looks up a symbol by name in all accessible scopes
    pub fn lookup(&mut self, name: &str) -> AnalyzerResult<Arc<AnalysisSymbol>> {
        self.stats.total_lookups += 1;
        
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.iter().find(|s| s.name == name) {
                self.stats.successful_lookups += 1;
                return Ok(symbol.clone());
            }
        }
        
        self.stats.failed_lookups += 1;
        self.stats.unresolved_references += 1;
        
        Err(AnalysisError::symbol_error(format!(
            "Symbol {} not found in any scope",
            name
        ))).context("Symbol lookup failed")
    }

    /// Gets statistics about symbol table operations
    pub fn get_stats(&self) -> &SymbolTableStats {
        &self.stats
    }
}
