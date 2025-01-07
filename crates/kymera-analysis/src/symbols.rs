use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{AnalysisError, Result};
use crate::types::Type;

/// Core analysis symbol representation
#[derive(Debug, Clone)]
pub struct AnalysisSymbol {
    /// Name of the symbol
    pub name: String,
    /// Type information
    pub ty: Type,
    /// Analysis scope level
    pub scope_level: usize,
    /// Mutability for analysis
    pub is_mutable: bool,
    /// Symbol documentation
    pub documentation: Option<String>,
    /// Analysis-specific metadata
    pub metadata: SymbolMetadata,
}

/// Analysis-specific metadata for symbols
#[derive(Debug, Clone)]
pub struct SymbolMetadata {
    /// Whether the symbol has been type-checked
    pub type_checked: bool,
    /// Whether all references have been resolved
    pub references_resolved: bool,
    /// Whether the symbol is used
    pub is_used: bool,
    /// Source location information
    pub location: SourceLocation,
}

/// Source location tracking
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// Core analysis scope management
#[derive(Debug, Default)]
pub struct AnalysisScope {
    /// Symbols in this scope
    symbols: HashMap<String, AnalysisSymbol>,
    /// Analysis-specific scope data
    scope_data: ScopeData,
}

/// Analysis-specific scope information
#[derive(Debug, Default)]
pub struct ScopeData {
    /// Whether this scope has been fully analyzed
    pub analyzed: bool,
    /// Number of references to symbols in this scope
    pub reference_count: usize,
    /// Whether this scope contains unsafe code
    pub contains_unsafe: bool,
    /// Whether this scope has side effects
    pub has_side_effects: bool,
}

/// Core analysis symbol table
#[derive(Debug, Default)]
pub struct AnalysisTable {
    /// Stack of analysis scopes
    scopes: Vec<AnalysisScope>,
    /// Current analysis level
    current_level: usize,
    /// Analysis statistics
    stats: AnalysisStats,
}

/// Analysis statistics tracking
#[derive(Debug, Default)]
pub struct AnalysisStats {
    pub total_symbols: usize,
    pub resolved_types: usize,
    pub unresolved_references: usize,
    pub unsafe_blocks: usize,
}

impl AnalysisTable {
    /// Creates a new analysis table
    pub fn new() -> Self {
        let mut table = Self::default();
        table.push_scope(); // Global scope
        table
    }

    /// Enters a new analysis scope
    pub fn push_scope(&mut self) {
        self.scopes.push(AnalysisScope::default());
        self.current_level += 1;
    }

    /// Exits the current analysis scope
    pub fn pop_scope(&mut self) -> Result<()> {
        if self.current_level == 0 {
            return Err(AnalysisError::ScopeError("Cannot pop global scope".to_string()));
        }
        
        // Update statistics before popping
        if let Some(scope) = self.scopes.last() {
            if scope.scope_data.contains_unsafe {
                self.stats.unsafe_blocks += 1;
            }
        }
        
        self.scopes.pop();
        self.current_level -= 1;
        Ok(())
    }

    /// Defines a new symbol for analysis
    pub fn define(&mut self, symbol: AnalysisSymbol) -> Result<()> {
        let scope = self.scopes.last_mut()
            .ok_or_else(|| AnalysisError::InternalError("No active scope".to_string()))?;

        if scope.symbols.contains_key(&symbol.name) {
            return Err(AnalysisError::SemanticError(
                format!("Symbol '{}' already defined in current scope", symbol.name)
            ));
        }

        scope.symbols.insert(symbol.name.clone(), symbol);
        self.stats.total_symbols += 1;
        Ok(())
    }

    /// Looks up a symbol for analysis
    pub fn lookup(&self, name: &str) -> Result<Arc<AnalysisSymbol>> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Ok(Arc::new(symbol.clone()));
            }
        }
        self.stats.unresolved_references += 1;
        Err(AnalysisError::SymbolNotFound(name.to_string()))
    }

    /// Updates analysis information for a symbol
    pub fn update(&mut self, name: &str, new_symbol: AnalysisSymbol) -> Result<()> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(existing) = scope.symbols.get_mut(name) {
                if !existing.is_mutable {
                    return Err(AnalysisError::SemanticError(
                        format!("Cannot modify immutable symbol '{}'", name)
                    ));
                }
                *existing = new_symbol;
                if existing.metadata.type_checked {
                    self.stats.resolved_types += 1;
                }
                return Ok(());
            }
        }
        Err(AnalysisError::SymbolNotFound(name.to_string()))
    }

    /// Gets all symbols in current analysis scope
    pub fn current_scope_symbols(&self) -> Result<Vec<Arc<AnalysisSymbol>>> {
        let scope = self.scopes.last()
            .ok_or_else(|| AnalysisError::InternalError("No active scope".to_string()))?;
        
        Ok(scope.symbols.values()
            .map(|s| Arc::new(s.clone()))
            .collect())
    }

    /// Gets the current analysis scope level
    pub fn current_level(&self) -> usize {
        self.current_level
    }

    /// Gets current analysis statistics
    pub fn get_stats(&self) -> &AnalysisStats {
        &self.stats
    }
}
