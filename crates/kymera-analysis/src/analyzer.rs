use kymera_parser::ast::*;
use std::sync::Arc;

use kymera_parser::ast::*;
use salsa::ParallelDatabase;

use crate::error::{AnalysisError, Result};
use crate::symbols::{Symbol, SymbolTable};
use crate::types::{Type, TypeChecker};

/// Main analyzer for Kymera code
#[derive(Debug)]
pub struct Analyzer {
    /// Symbol table for name resolution
    symbols: SymbolTable,
    /// Type checker for type inference and validation
    type_checker: TypeChecker,
}

impl Analyzer {
    /// Creates a new analyzer
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            type_checker: TypeChecker::new(),
        }
    }

    /// Analyzes a complete AST
    pub fn analyze(&mut self, ast: &[ASTNode]) -> Result<()> {
        // First pass: collect declarations
        self.collect_declarations(ast)?;
        
        // Second pass: analyze expressions and statements
        self.analyze_nodes(ast)?;
        
        Ok(())
    }

    /// First pass: collect all declarations to build symbol table
    fn collect_declarations(&mut self, nodes: &[ASTNode]) -> Result<()> {
        for node in nodes {
            match node {
                ASTNode::FunctionDecl(func) => {
                    let symbol = Symbol {
                        name: func.name.clone(),
                        ty: self.function_type(func)?,
                        scope_level: self.symbols.current_level(),
                        is_mutable: false,
                        documentation: func.doc_comment.clone(),
                    };
                    self.symbols.define(symbol)?;
                },
                ASTNode::StructDecl(struct_def) => {
                    let symbol = Symbol {
                        name: struct_def.name.clone(),
                        ty: self.struct_type(struct_def)?,
                        scope_level: self.symbols.current_level(),
                        is_mutable: false,
                        documentation: struct_def.doc_comment.clone(),
                    };
                    self.symbols.define(symbol)?;
                },
                ASTNode::EnumDecl(enum_def) => {
                    let symbol = Symbol {
                        name: enum_def.name.clone(),
                        ty: self.enum_type(enum_def)?,
                        scope_level: self.symbols.current_level(),
                        is_mutable: false,
                        documentation: enum_def.doc_comment.clone(),
                    };
                    self.symbols.define(symbol)?;
                },
                _ => continue,
            }
        }
        Ok(())
    }

    /// Second pass: analyze all nodes
    fn analyze_nodes(&mut self, nodes: &[ASTNode]) -> Result<()> {
        for node in nodes {
            self.analyze_node(node)?;
        }
        Ok(())
    }

    /// Analyzes a single AST node
    fn analyze_node(&mut self, node: &ASTNode) -> Result<Type> {
        match node {
            ASTNode::FunctionDecl(func) => self.analyze_function(func),
            ASTNode::StructDecl(struct_def) => self.analyze_struct(struct_def),
            ASTNode::EnumDecl(enum_def) => self.analyze_enum(enum_def),
            ASTNode::VarDecl(var) => self.analyze_var_decl(var),
            ASTNode::Block(statements) => self.analyze_block(statements),
            ASTNode::Expression(expr) => self.analyze_expression(expr),
            // Add more node types as needed
        }
    }

    /// Analyzes a function declaration
    fn analyze_function(&mut self, func: &FunctionDecl) -> Result<Type> {
        // Push new scope for function body
        self.symbols.push_scope();

        // Add parameters to scope
        for param in &func.parameters {
            let param_type = self.resolve_type(&param.type_annotation)?;
            let symbol = Symbol {
                name: param.name.clone(),
                ty: param_type,
                scope_level: self.symbols.current_level(),
                is_mutable: false,
                documentation: None,
            };
            self.symbols.define(symbol)?;
        }

        // Analyze function body
        let body_type = self.analyze_block(&func.body)?;
        
        // Check return type matches
        let declared_return = self.resolve_type(&func.return_type)?;
        if !self.type_checker.can_coerce(&body_type, &declared_return) {
            return Err(AnalysisError::TypeError(format!(
                "Function '{}' declares return type {} but returns {}",
                func.name, declared_return, body_type
            )));
        }

        // Pop function scope
        self.symbols.pop_scope()?;

        Ok(declared_return)
    }

    /// Analyzes a block of statements
    fn analyze_block(&mut self, statements: &[ASTNode]) -> Result<Type> {
        self.symbols.push_scope();
        
        let mut block_type = Type::Unit;
        
        for stmt in statements {
            block_type = self.analyze_node(stmt)?;
        }
        
        self.symbols.pop_scope()?;
        
        Ok(block_type)
    }

    /// Resolves a type annotation to a concrete type
    fn resolve_type(&self, annotation: &TypeAnnotation) -> Result<Type> {
        match annotation {
            TypeAnnotation::Simple(name) => {
                match name.as_str() {
                    "unit" => Ok(Type::Unit),
                    "bool" => Ok(Type::Bool),
                    "int" => Ok(Type::Int),
                    "float" => Ok(Type::Float),
                    "string" => Ok(Type::String),
                    _ => {
                        // Look up user-defined type
                        let symbol = self.symbols.lookup(name)?;
                        Ok(symbol.ty.clone())
                    }
                }
            },
            TypeAnnotation::Array(inner) => {
                let inner_type = self.resolve_type(inner)?;
                Ok(Type::Array(Box::new(inner_type)))
            },
            TypeAnnotation::Option(inner) => {
                let inner_type = self.resolve_type(inner)?;
                Ok(Type::Option(Box::new(inner_type)))
            },
            TypeAnnotation::Result(ok, err) => {
                let ok_type = self.resolve_type(ok)?;
                let err_type = self.resolve_type(err)?;
                Ok(Type::Result(Box::new(ok_type), Box::new(err_type)))
            },
            // Add more type annotations as needed
        }
    }
}
