use anyhow::{Context, Result as AnalyzerResult};
use kymera_parser::ast::{
    AstNode, Expression, Statement, Function, Struct, Enum, Declaration, Assignment,
};

use crate::err::AnalysisError;
use crate::types::{Type, TypeChecker, FunctionType, StructType, EnumType};
use crate::symbols::{AnalysisSymbol, AnalysisTable, SymbolKind, Visibility};

/// Main analyzer for Kymera code
#[derive(Debug)]
pub struct Analyzer {
    /// Symbol table for name resolution
    symbols: AnalysisTable,
    /// Type checker for type inference and validation
    type_checker: TypeChecker,
}

impl Analyzer {
    /// Creates a new analyzer
    pub fn new() -> Self {
        Self {
            symbols: AnalysisTable::new(),
            type_checker: TypeChecker::new(),
        }
    }

    /// Analyzes a complete AST
    pub fn analyze(&mut self, ast: &[AstNode]) -> AnalyzerResult<()> {
        // First pass: collect declarations
        self.collect_declarations(ast)
            .context("Failed during declaration collection")?;
        
        // Second pass: analyze expressions and statements
        self.analyze_nodes(ast)
            .context("Failed during node analysis")?;
        
        Ok(())
    }

    /// First pass: collect all declarations to build symbol table
    fn collect_declarations(&mut self, nodes: &[AstNode]) -> AnalyzerResult<()> {
        for node in nodes {
            if let AstNode::Statement(stmt) = node {
                match stmt {
                    Statement::Function(func) => {
                        let symbol = AnalysisSymbol {
                            name: func.name.clone(),
                            kind: SymbolKind::Function,
                            ty: self.function_type(func)
                                .context("Failed to determine function type")?,
                            scope_level: self.symbols.current_level(),
                            documentation: None,
                            metadata: Default::default(),
                            visibility: Visibility::Public,
                            is_mutable: false,
                        };
                        self.symbols.define(symbol)
                            .with_context(|| format!("Failed to define function symbol: {}", func.name))?;
                    },
                    Statement::Struct(struct_def) => {
                        let symbol = AnalysisSymbol {
                            name: struct_def.name.clone(),
                            kind: SymbolKind::Type,
                            ty: self.struct_type(struct_def)
                                .context("Failed to determine struct type")?,
                            scope_level: self.symbols.current_level(),
                            documentation: None,
                            metadata: Default::default(),
                            visibility: Visibility::Public,
                            is_mutable: false,
                        };
                        self.symbols.define(symbol)
                            .with_context(|| format!("Failed to define struct symbol: {}", struct_def.name))?;
                    },
                    Statement::Enum(enum_def) => {
                        let symbol = AnalysisSymbol {
                            name: enum_def.name.clone(),
                            kind: SymbolKind::Type,
                            ty: self.enum_type(enum_def)
                                .context("Failed to determine enum type")?,
                            scope_level: self.symbols.current_level(),
                            documentation: None,
                            metadata: Default::default(),
                            visibility: Visibility::Public,
                            is_mutable: false,
                        };
                        self.symbols.define(symbol)
                            .with_context(|| format!("Failed to define enum symbol: {}", enum_def.name))?;
                    },
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    /// Second pass: analyze all nodes
    fn analyze_nodes(&mut self, nodes: &[AstNode]) -> AnalyzerResult<()> {
        for node in nodes {
            self.analyze_node(node)
                .with_context(|| format!("Failed to analyze node: {:?}", node))?;
        }
        Ok(())
    }

    /// Analyzes a single AST node
    fn analyze_node(&mut self, node: &AstNode) -> AnalyzerResult<Type> {
        match node {
            AstNode::Statement(stmt) => match stmt {
                Statement::Function(func) => self.analyze_function(func)
                    .with_context(|| format!("Failed to analyze function: {}", func.name)),
                Statement::Struct(struct_def) => self.analyze_struct(struct_def)
                    .with_context(|| format!("Failed to analyze struct: {}", struct_def.name)),
                Statement::Enum(enum_def) => self.analyze_enum(enum_def)
                    .with_context(|| format!("Failed to analyze enum: {}", enum_def.name)),
                Statement::Declaration(decl) => self.analyze_declaration(decl)
                    .with_context(|| format!("Failed to analyze declaration: {}", decl.name)),
                Statement::Assignment(assign) => self.analyze_assignment(assign)
                    .with_context(|| format!("Failed to analyze assignment to: {}", assign.name)),
                Statement::Block(statements, _) => self.analyze_block(statements)
                    .context("Failed to analyze block"),
                Statement::Expression(expr) => self.analyze_expression(expr)
                    .context("Failed to analyze expression"),
                _ => Ok(Type::Unit), // Other statement types return unit
            },
            AstNode::Expression(expr) => self.analyze_expression(expr)
                .context("Failed to analyze expression"),
        }
    }

    /// Analyzes a function declaration
    fn analyze_function(&mut self, func: &Function) -> AnalyzerResult<Type> {
        // Push new scope for function body
        self.symbols.push_scope();

        // Add parameters to scope
        for param in &func.params {
            let symbol = AnalysisSymbol {
                name: param.clone(),
                kind: SymbolKind::Parameter,
                ty: Type::Unknown, // Parameters have unknown type until type inference
                scope_level: self.symbols.current_level(),
                documentation: None,
                metadata: Default::default(),
                visibility: Visibility::Private,
                is_mutable: false,
            };
            self.symbols.define(symbol)
                .with_context(|| format!("Failed to define parameter symbol: {}", param))?;
        }

        // Analyze function body
        let mut body_type = Type::Unit;
        for stmt in &func.body {
            body_type = self.analyze_node(stmt)?;
        }

        // Pop function scope
        self.symbols.pop_scope()
            .context("Failed to pop function scope")?;

        Ok(body_type)
    }

    /// Analyzes a block of statements
    fn analyze_block(&mut self, statements: &[AstNode]) -> AnalyzerResult<Type> {
        self.symbols.push_scope();
        
        let mut block_type = Type::Unit;
        
        for stmt in statements {
            block_type = self.analyze_node(stmt)?;
        }
        
        self.symbols.pop_scope()?;
        
        Ok(block_type)
    }

    /// Derives the type of a function declaration
    fn function_type(&mut self, func: &Function) -> AnalyzerResult<Type> {
        // For now, all functions return Unit and take Unknown type parameters
        let param_types = vec![Type::Unknown; func.params.len()];
        let return_type = Type::Unit;

        Ok(Type::Function(FunctionType {
            params: param_types,
            return_type: Box::new(return_type),
            type_params: vec![],
        }))
    }

    /// Derives the type of a struct declaration
    fn struct_type(&mut self, struct_def: &Struct) -> AnalyzerResult<Type> {
        let mut fields = Vec::new();
        
        // Parse field types from the type strings
        for (name, type_str) in &struct_def.fields {
            let field_type = Type::parse(type_str)
                .with_context(|| format!("Failed to parse field type for {}: {}", name, type_str))?;
            fields.push((name.clone(), field_type));
        }
        
        Ok(Type::Struct(StructType {
            name: struct_def.name.clone(),
            fields,
            type_params: vec![],
        }))
    }

    /// Derives the type of an enum declaration
    fn enum_type(&mut self, enum_def: &Enum) -> AnalyzerResult<Type> {
        let variants = enum_def.variants.iter()
            .map(|name| (name.clone(), None))
            .collect();
        
        Ok(Type::Enum(EnumType {
            name: enum_def.name.clone(),
            variants,
            type_params: vec![],
        }))
    }

    /// Analyzes a struct declaration
    fn analyze_struct(&mut self, struct_def: &Struct) -> AnalyzerResult<Type> {
        self.struct_type(struct_def)
    }

    /// Analyzes an enum declaration
    fn analyze_enum(&mut self, enum_def: &Enum) -> AnalyzerResult<Type> {
        self.enum_type(enum_def)
    }

    /// Analyzes a variable declaration
    fn analyze_declaration(&mut self, decl: &Declaration) -> AnalyzerResult<Type> {
        let var_type = self.type_checker.infer_literal(&decl.value);
        
        let symbol = AnalysisSymbol {
            name: decl.name.clone(),
            kind: SymbolKind::Variable,
            ty: var_type.clone(),
            scope_level: self.symbols.current_level(),
            documentation: None,
            metadata: Default::default(),
            visibility: Visibility::Private,
            is_mutable: false,
        };
        self.symbols.define(symbol)
            .with_context(|| format!("Failed to define variable symbol: {}", decl.name))?;
        
        Ok(var_type)
    }

    /// Analyzes an assignment
    fn analyze_assignment(&mut self, assign: &Assignment) -> AnalyzerResult<Type> {
        let symbol = self.symbols.lookup(&assign.name)?;
        let value_type = self.analyze_node(&assign.value)?;
        
        if !symbol.is_mutable {
            return Err(AnalysisError::semantic_error(format!(
                "Cannot assign to immutable variable {}",
                assign.name
            ))).context("Assignment to immutable variable");
        }
        
        if !self.type_checker.can_coerce(&value_type, &symbol.ty) {
            return Err(AnalysisError::type_error(format!(
                "Cannot assign value of type {} to variable {} of type {}",
                value_type, assign.name, symbol.ty
            ))).context("Type mismatch in assignment");
        }
        
        Ok(Type::Unit)
    }

    /// Analyzes an expression
    fn analyze_expression(&mut self, expr: &Expression) -> AnalyzerResult<Type> {
        match expr {
            Expression::Literal(lit) => Ok(self.type_checker.infer_literal(lit)),
            Expression::Identifier(name, _) => {
                let symbol = self.symbols.lookup(name)?;
                Ok(symbol.ty.clone())
            },
            Expression::BinaryOp(op) => {
                let left_type = self.analyze_node(&op.left)?;
                let right_type = self.analyze_node(&op.right)?;
                self.type_checker.check_binary_op(&left_type, &op.op, &right_type)
            },
            Expression::UnaryOp(op) => {
                let expr_type = self.analyze_node(&op.operand)?;
                self.type_checker.check_unary_op(&op.op, &expr_type)
            },
            Expression::FunctionCall(call) => {
                let callee_symbol = self.symbols.lookup(&call.name)?;
                match &callee_symbol.ty {
                    Type::Function(ft) => {
                        if call.args.len() != ft.params.len() {
                            return Err(AnalysisError::type_error(format!(
                                "Function {} expects {} arguments but got {}",
                                call.name, ft.params.len(), call.args.len()
                            ))).context("Argument count mismatch");
                        }
                        for (arg, expected_type) in call.args.iter().zip(ft.params.iter()) {
                            let arg_type = self.analyze_node(arg)?;
                            if !self.type_checker.can_coerce(&arg_type, expected_type) {
                                return Err(AnalysisError::type_error(format!(
                                    "Argument type mismatch: expected {}, got {}",
                                    expected_type, arg_type
                                ))).context("Argument type mismatch");
                            }
                        }
                        Ok(*ft.return_type.clone())
                    },
                    _ => Err(AnalysisError::type_error(format!(
                        "{} is not a function", call.name
                    ))).context("Not a function"),
                }
            },
            Expression::FieldAccess(struct_name, field_name, _) => {
                let struct_symbol = self.symbols.lookup(struct_name)?;
                match &struct_symbol.ty {
                    Type::Struct(s) => {
                        if let Some((_, field_type)) = s.fields.iter().find(|(name, _)| name == field_name) {
                            Ok(field_type.clone())
                        } else {
                            Err(AnalysisError::type_error(format!(
                                "Field {} not found in struct {}", field_name, struct_name
                            ))).context("Field not found")
                        }
                    },
                    _ => Err(AnalysisError::type_error(format!(
                        "{} is not a struct", struct_name
                    ))).context("Not a struct"),
                }
            },
            Expression::ArrayAccess(array_name, index_expr, _) => {
                let array_symbol = self.symbols.lookup(array_name)?;
                match &array_symbol.ty {
                    Type::Array(element_type) => {
                        let index_type = self.analyze_node(index_expr)?;
                        if index_type != Type::Int {
                            return Err(AnalysisError::type_error(format!(
                                "Array index must be an integer, got {}", index_type
                            ))).context("Invalid array index");
                        }
                        Ok(*element_type.clone())
                    },
                    _ => Err(AnalysisError::type_error(format!(
                        "{} is not an array", array_name
                    ))).context("Not an array"),
                }
            },
        }
    }
}
