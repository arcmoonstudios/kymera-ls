//! src/analysis/symbols.rs
//! Symbol table and scope management for Kymera.

use std::collections::HashMap;

/// Represents a symbol table for managing scopes and symbols.
#[derive(Debug, Default)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize, // Tracks the current scope index
}

/// Represents a scope in the symbol table.
#[derive(Debug)]
struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<usize>, // Index of the parent scope
}

/// Represents a symbol in the symbol table.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_info: Option<String>, // Optional type information
    pub is_mutable: bool,         // Whether the symbol is mutable
    pub is_defined: bool,         // Whether the symbol is fully defined
    pub visibility: Visibility,   // Visibility of the symbol
    pub documentation: Option<String>, // Documentation for the symbol
}

/// Represents the kind of a symbol.
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    // Variables and Constants
    Variable,
    Constant,
    GlobalVariable,
    GlobalConstant,

    // Functions and Methods
    Function,
    Method,
    Constructor,
    Destructor,
    Closure,
    Lambda,

    // Types
    Type,
    PrimitiveType,
    AliasType,
    GenericType,
    Enum,
    Union,
    Trait,
    Protocol,

    // Modules and Namespaces
    Module,
    Package,

    // Parameters and Arguments
    Parameter,
    VariadicParameter,

    // Control Flow
    Label,
    LoopVariable,

    // Macros and Templates
    Macro,
    Template,

    // Attributes and Annotations
    Attribute,
    Decorator,

    // Imports and Exports
    Import,
    Export,

    // Error Handling
    ErrorType,
    Exception,

    // Miscellaneous
    Placeholder,
    Builtin,
    Unknown,
}

/// Represents the visibility of a symbol.
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Errors that can occur during symbol table operations.
#[derive(Debug, PartialEq)]
pub enum SymbolError {
    DuplicateSymbol(String),
    SymbolNotFound(String),
    InvalidScope,
}

impl SymbolTable {
    /// Creates a new, empty symbol table with a global scope.
    pub fn new() -> Self {
        let global_scope = Scope {
            symbols: HashMap::new(),
            parent: None,
        };
        SymbolTable {
            scopes: vec![global_scope],
            current_scope: 0, // Start with the global scope
        }
    }

    /// Enters a new scope, optionally with a parent scope.
    pub fn enter_scope(&mut self) -> usize {
        let new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope),
        };
        self.scopes.push(new_scope);
        self.current_scope = self.scopes.len() - 1; // Update current scope
        self.current_scope
    }

    /// Exits the current scope and returns to the parent scope.
    pub fn exit_scope(&mut self) -> Result<(), SymbolError> {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
            Ok(())
        } else {
            Err(SymbolError::InvalidScope)
        }
    }

    /// Adds a symbol to the current scope.
    pub fn add_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        type_info: Option<String>,
        is_mutable: bool,
        visibility: Visibility,
        documentation: Option<String>,
    ) -> Result<(), SymbolError> {
        if self.symbol_exists_in_current_scope(&name) {
            return Err(SymbolError::DuplicateSymbol(name));
        }
        let symbol = Symbol {
            name: name.clone(),
            kind,
            type_info,
            is_mutable,
            is_defined: false,
            visibility,
            documentation,
        };
        self.scopes[self.current_scope].symbols.insert(name, symbol);
        Ok(())
    }

    /// Marks a symbol as defined.
    pub fn define_symbol(&mut self, name: &str) -> Result<(), SymbolError> {
        if let Some(symbol) = self.scopes[self.current_scope].symbols.get_mut(name) {
            symbol.is_defined = true;
            Ok(())
        } else {
            Err(SymbolError::SymbolNotFound(name.to_string()))
        }
    }

    /// Looks up a symbol by name, starting from the current scope and moving up through parent scopes.
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        let mut scope_index = self.current_scope;
        loop {
            let scope = &self.scopes[scope_index];
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
            match scope.parent {
                Some(parent_index) => scope_index = parent_index,
                None => break,
            }
        }
        None
    }

    /// Checks if a symbol exists in the current scope.
    pub fn symbol_exists_in_current_scope(&self, name: &str) -> bool {
        self.scopes[self.current_scope].symbols.contains_key(name)
    }

    /// Returns the current scope index.
    pub fn current_scope(&self) -> usize {
        self.current_scope
    }

    /// Returns the number of scopes in the symbol table.
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    /// Updates the documentation for a symbol.
    pub fn update_documentation(&mut self, name: &str, documentation: String) -> Result<(), SymbolError> {
        if let Some(symbol) = self.scopes[self.current_scope].symbols.get_mut(name) {
            symbol.documentation = Some(documentation);
            Ok(())
        } else {
            Err(SymbolError::SymbolNotFound(name.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();

        // Add a symbol to the global scope
        assert!(symbol_table
            .add_symbol(
                "x".to_string(),
                SymbolKind::Variable,
                Some("i32".to_string()),
                false,
                Visibility::Public,
                None,
            )
            .is_ok());

        // Enter a nested scope
        let nested_scope = symbol_table.enter_scope();
        assert_eq!(nested_scope, 1); // Verify it's the second scope (after global)

        // Add a symbol to the nested scope
        assert!(symbol_table
            .add_symbol(
                "y".to_string(),
                SymbolKind::Variable,
                Some("f64".to_string()),
                true,
                Visibility::Private,
                None,
            )
            .is_ok());

        // Lookup symbols
        assert!(symbol_table.lookup_symbol("x").is_some()); // Found in global scope
        assert!(symbol_table.lookup_symbol("y").is_some()); // Found in nested scope
        assert!(symbol_table.lookup_symbol("z").is_none()); // Not found

        // Ensure symbol exists in the current scope
        assert!(symbol_table.symbol_exists_in_current_scope("y"));
        assert!(!symbol_table.symbol_exists_in_current_scope("x"));

        // Exit the nested scope
        assert!(symbol_table.exit_scope().is_ok());

        // Ensure the nested scope's symbols are no longer accessible
        assert!(!symbol_table.symbol_exists_in_current_scope("y"));
    }

    #[test]
    fn test_duplicate_symbol() {
        let mut symbol_table = SymbolTable::new();

        // Add a symbol to the global scope
        assert!(symbol_table
            .add_symbol(
                "x".to_string(),
                SymbolKind::Variable,
                Some("i32".to_string()),
                false,
                Visibility::Public,
                None,
            )
            .is_ok());

        // Attempt to add a duplicate symbol
        assert_eq!(
            symbol_table.add_symbol(
                "x".to_string(),
                SymbolKind::Variable,
                Some("i32".to_string()),
                false,
                Visibility::Public,
                None,
            ),
            Err(SymbolError::DuplicateSymbol("x".to_string()))
        );
    }

    #[test]
    fn test_define_symbol() {
        let mut symbol_table = SymbolTable::new();

        // Add a symbol to the global scope
        assert!(symbol_table
            .add_symbol(
                "x".to_string(),
                SymbolKind::Variable,
                Some("i32".to_string()),
                false,
                Visibility::Public,
                None,
            )
            .is_ok());

        // Mark the symbol as defined
        assert!(symbol_table.define_symbol("x").is_ok());

        // Lookup the symbol and check if it's defined
        let symbol = symbol_table.lookup_symbol("x").unwrap();
        assert!(symbol.is_defined);
    }

    #[test]
    fn test_invalid_scope_exit() {
        let mut symbol_table = SymbolTable::new();

        // Attempt to exit the global scope
        assert_eq!(symbol_table.exit_scope(), Err(SymbolError::InvalidScope));
    }
}