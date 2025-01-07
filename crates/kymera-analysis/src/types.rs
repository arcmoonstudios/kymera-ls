use std::fmt;
use std::sync::Arc;
use anyhow::{Context, Result as AnalyzerResult};
use kymera_parser::ast::Literal;

use crate::err::AnalysisError;

/// Represents a type parameter constraint
#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    /// Type must implement a trait
    Trait(String),
    /// Type must be a subtype of another type
    Subtype(Box<Type>),
    /// Type must be one of a set of types
    OneOf(Vec<Type>),
}

/// Represents a type parameter with optional constraints
#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    /// Name of the type parameter
    pub name: String,
    /// Optional constraints on the type parameter
    pub constraints: Vec<TypeConstraint>,
    /// Default type if not specified
    pub default_type: Option<Box<Type>>,
}

/// Represents a type in the Kymera type system
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Built-in primitive types
    Unit,
    Bool,
    Int,
    Float,
    String,
    
    /// Container types
    Array(Box<Type>),
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    
    /// User-defined types
    Struct(StructType),
    Enum(EnumType),
    Function(FunctionType),
    
    /// Special types
    Generic(String),
    Unknown, // Used during type inference
}

/// Represents a struct type
#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, Type)>,
    pub type_params: Vec<String>,
}

/// Represents an enum type
#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
    pub name: String,
    pub variants: Vec<(String, Option<Type>)>,
    pub type_params: Vec<String>,
}

/// Represents a function type
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
    pub type_params: Vec<String>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit => write!(f, "()"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Array(t) => write!(f, "[{}]", t),
            Type::Option(t) => write!(f, "Option<{}>", t),
            Type::Result(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            Type::Struct(s) => write!(f, "{}", s.name),
            Type::Enum(e) => write!(f, "{}", e.name),
            Type::Function(ft) => {
                write!(f, "fn(")?;
                for (i, param) in ft.params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")? }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ft.return_type)
            },
            Type::Generic(name) => write!(f, "{}", name),
            Type::Unknown => write!(f, "<unknown>"),
        }
    }
}

/// Type inference and checking functionality
#[derive(Debug, Default)]
pub struct TypeChecker {
    type_env: Vec<(String, Arc<Type>)>,
}

impl TypeChecker {
    /// Creates a new type checker
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if a binary operation is valid and returns its result type
    pub fn check_binary_op(&self, left: &Type, op: &str, right: &Type) -> AnalyzerResult<Type> {
        match op {
            "+" | "-" | "*" | "/" | "%" => {
                match (left, right) {
                    (Type::Int, Type::Int) => Ok(Type::Int),
                    (Type::Float, Type::Float) => Ok(Type::Float),
                    (Type::Int, Type::Float) | (Type::Float, Type::Int) => Ok(Type::Float),
                    _ => Err(AnalysisError::type_error(format!(
                        "Invalid operands for arithmetic operation: {} {} {}",
                        left, op, right
                    ))).context("Invalid arithmetic operands"),
                }
            },
            "==" | "!=" => {
                if self.can_coerce(left, right) || self.can_coerce(right, left) {
                    Ok(Type::Bool)
                } else {
                    Err(AnalysisError::type_error(format!(
                        "Cannot compare values of types {} and {}",
                        left, right
                    ))).context("Invalid comparison operands")
                }
            },
            "<" | "<=" | ">" | ">=" => {
                match (left, right) {
                    (Type::Int, Type::Int) | (Type::Float, Type::Float) => Ok(Type::Bool),
                    (Type::Int, Type::Float) | (Type::Float, Type::Int) => Ok(Type::Bool),
                    _ => Err(AnalysisError::type_error(format!(
                        "Invalid operands for comparison: {} {} {}",
                        left, op, right
                    ))).context("Invalid comparison operands"),
                }
            },
            "&&" | "||" => {
                if left == &Type::Bool && right == &Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(AnalysisError::type_error(format!(
                        "Logical operations require boolean operands, got {} and {}",
                        left, right
                    ))).context("Invalid logical operands")
                }
            },
            _ => Err(AnalysisError::type_error(format!(
                "Unknown binary operator: {}",
                op
            ))).context("Unknown operator"),
        }
    }

    /// Checks if a unary operation is valid and returns its result type
    pub fn check_unary_op(&self, op: &str, expr: &Type) -> AnalyzerResult<Type> {
        match op {
            "-" => {
                match expr {
                    Type::Int | Type::Float => Ok(expr.clone()),
                    _ => Err(AnalysisError::type_error(format!(
                        "Cannot negate value of type {}",
                        expr
                    ))).context("Invalid negation operand"),
                }
            },
            "!" => {
                if expr == &Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(AnalysisError::type_error(format!(
                        "Logical not requires boolean operand, got {}",
                        expr
                    ))).context("Invalid logical not operand")
                }
            },
            _ => Err(AnalysisError::type_error(format!(
                "Unknown unary operator: {}",
                op
            ))).context("Unknown operator"),
        }
    }

    /// Infers the type of a literal
    pub fn infer_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(..) => Type::Int,
            Literal::Float(..) => Type::Float,
            Literal::Bool(..) => Type::Bool,
            Literal::Strng(..) => Type::String,
            Literal::Stilo(..) => Type::String,
            Literal::Nil(..) => Type::Unit,
        }
    }

    /// Resolves a type variable to its concrete type
    pub fn resolve_type_var(&self, name: &str) -> AnalyzerResult<Arc<Type>> {
        self.type_env.iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, t)| t.clone())
            .ok_or_else(|| AnalysisError::type_error(format!("Unresolved type variable: {}", name)))
            .context("Type resolution failed")
    }

    /// Binds a type variable to a concrete type
    pub fn bind_type_var(&mut self, name: String, ty: Type) -> AnalyzerResult<()> {
        if self.type_env.iter().any(|(n, _)| n == &name) {
            return Err(AnalysisError::type_error(format!(
                "Type variable {} already bound", name
            ))).context("Type binding failed");
        }
        self.type_env.push((name, Arc::new(ty)));
        Ok(())
    }

    /// Checks if one type can be coerced into another
    pub fn can_coerce(&self, from: &Type, to: &Type) -> bool {
        match (from, to) {
            // Same types can always be coerced
            (t1, t2) if t1 == t2 => true,
            
            // Int can be coerced to Float
            (Type::Int, Type::Float) => true,
            
            // Array coercion is covariant
            (Type::Array(t1), Type::Array(t2)) => self.can_coerce(t1, t2),
            
            // Option coercion is covariant
            (Type::Option(t1), Type::Option(t2)) => self.can_coerce(t1, t2),
            
            // Result is covariant in Ok type and contravariant in Err type
            (Type::Result(ok1, err1), Type::Result(ok2, err2)) => 
                self.can_coerce(ok1, ok2) && self.can_coerce(err2, err1),
            
            // Function types follow standard function subtyping rules
            (Type::Function(f1), Type::Function(f2)) => {
                if f1.params.len() != f2.params.len() {
                    return false;
                }
                
                // Parameters are contravariant
                for (p1, p2) in f1.params.iter().zip(f2.params.iter()) {
                    if !self.can_coerce(p2, p1) {
                        return false;
                    }
                }
                
                // Return type is covariant
                self.can_coerce(&f1.return_type, &f2.return_type)
            },
            
            // Struct subtyping based on field types
            (Type::Struct(s1), Type::Struct(s2)) if s1.name == s2.name => {
                if s1.fields.len() != s2.fields.len() {
                    return false;
                }
                
                for ((n1, t1), (n2, t2)) in s1.fields.iter().zip(s2.fields.iter()) {
                    if n1 != n2 || !self.can_coerce(t1, t2) {
                        return false;
                    }
                }
                
                true
            },
            
            // Enum subtyping based on variant types
            (Type::Enum(e1), Type::Enum(e2)) if e1.name == e2.name => {
                if e1.variants.len() != e2.variants.len() {
                    return false;
                }
                
                for ((n1, t1), (n2, t2)) in e1.variants.iter().zip(e2.variants.iter()) {
                    if n1 != n2 {
                        return false;
                    }
                    match (t1, t2) {
                        (Some(t1), Some(t2)) => if !self.can_coerce(t1, t2) { return false; },
                        (None, None) => continue,
                        _ => return false,
                    }
                }
                
                true
            },
            
            _ => false,
        }
    }
}

impl Type {
    /// Parses a type string into a Type
    pub fn parse(type_str: &str) -> AnalyzerResult<Self> {
        let trimmed = type_str.trim();
        match trimmed {
            "()" => Ok(Type::Unit),
            "bool" => Ok(Type::Bool),
            "int" => Ok(Type::Int),
            "float" => Ok(Type::Float),
            "string" => Ok(Type::String),
            s if s.starts_with('[') && s.ends_with(']') => {
                let inner = &s[1..s.len()-1];
                let element_type = Type::parse(inner)
                    .with_context(|| format!("Failed to parse array element type: {}", inner))?;
                Ok(Type::Array(Box::new(element_type)))
            },
            s if s.starts_with("Option<") && s.ends_with('>') => {
                let inner = &s[7..s.len()-1];
                let inner_type = Type::parse(inner)
                    .with_context(|| format!("Failed to parse Option type parameter: {}", inner))?;
                Ok(Type::Option(Box::new(inner_type)))
            },
            s if s.starts_with("Result<") && s.ends_with('>') => {
                let inner = &s[7..s.len()-1];
                let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
                if parts.len() != 2 {
                    return Err(AnalysisError::type_parse_error(
                        "Result type requires exactly two type parameters",
                        type_str
                    )).context("Invalid Result type");
                }
                let ok_type = Type::parse(parts[0])
                    .with_context(|| format!("Failed to parse Result Ok type: {}", parts[0]))?;
                let err_type = Type::parse(parts[1])
                    .with_context(|| format!("Failed to parse Result Err type: {}", parts[1]))?;
                Ok(Type::Result(Box::new(ok_type), Box::new(err_type)))
            },
            s if s.starts_with("fn(") && s.contains(")") => {
                let (params_str, return_str) = s[3..].split_once(")").ok_or_else(|| 
                    AnalysisError::type_parse_error("Invalid function type syntax", type_str)
                )?;
                
                let return_type = if return_str.starts_with("->") {
                    Type::parse(return_str[2..].trim())
                        .with_context(|| format!("Failed to parse function return type: {}", return_str))?
                } else {
                    Type::Unit
                };

                let params = if params_str.trim().is_empty() {
                    Vec::new()
                } else {
                    params_str.split(',')
                        .map(|s| Type::parse(s.trim()))
                        .collect::<Result<Vec<_>, _>>()
                        .with_context(|| format!("Failed to parse function parameters: {}", params_str))?
                };

                Ok(Type::Function(FunctionType {
                    params,
                    return_type: Box::new(return_type),
                    type_params: Vec::new(),
                }))
            },
            s => {
                // Assume it's a named type (struct, enum, or generic)
                if s.chars().next().map_or(false, |c| c.is_uppercase()) {
                    if s.contains('<') {
                        // Parse generic type application
                        let (base, params) = s.split_once('<').ok_or_else(||
                            AnalysisError::type_parse_error("Invalid generic type syntax", type_str)
                        )?;
                        let params_str = &params[..params.len()-1];
                        let _type_params = params_str.split(',')
                            .map(|s| Type::parse(s.trim()))
                            .collect::<Result<Vec<_>, _>>()
                            .with_context(|| format!("Failed to parse type parameters: {}", params_str))?;
                        
                        // Return as generic application
                        Ok(Type::Generic(base.to_string()))
                    } else {
                        // Return as named type
                        Ok(Type::Generic(s.to_string()))
                    }
                } else {
                    Err(AnalysisError::type_parse_error(
                        "Unknown type",
                        type_str
                    )).context("Type parsing failed")
                }
            }
        }
    }

    /// Validates type parameters against their constraints
    pub fn validate_type_params(&self, type_params: &[TypeParameter]) -> AnalyzerResult<()> {
        match self {
            Type::Function(ft) => {
                for param in &ft.params {
                    param.validate_type_params(type_params)?;
                }
                ft.return_type.validate_type_params(type_params)?;
            },
            Type::Struct(st) => {
                for (_, field_type) in &st.fields {
                    field_type.validate_type_params(type_params)?;
                }
            },
            Type::Enum(et) => {
                for (_, variant_type) in &et.variants {
                    if let Some(t) = variant_type {
                        t.validate_type_params(type_params)?;
                    }
                }
            },
            Type::Generic(name) => {
                if !type_params.iter().any(|p| p.name == *name) {
                    return Err(AnalysisError::type_parameter_error(
                        "Undefined type parameter",
                        name
                    )).context("Type parameter validation failed");
                }
            },
            _ => {}
        }
        Ok(())
    }
}
