use std::fmt;
use std::sync::Arc;

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

    /// Unifies two types, returning the most specific type that matches both
    pub fn unify(&self, t1: &Type, t2: &Type) -> Option<Type> {
        match (t1, t2) {
            // Same types unify to themselves
            (t1, t2) if t1 == t2 => Some(t1.clone()),
            
            // Unknown can unify with anything
            (Type::Unknown, t) | (t, Type::Unknown) => Some(t.clone()),
            
            // Array types unify if their element types unify
            (Type::Array(t1), Type::Array(t2)) => 
                self.unify(t1, t2).map(|t| Type::Array(Box::new(t))),
            
            // Option types unify if their inner types unify
            (Type::Option(t1), Type::Option(t2)) =>
                self.unify(t1, t2).map(|t| Type::Option(Box::new(t))),
            
            // Result types unify if both their OK and Err types unify
            (Type::Result(ok1, err1), Type::Result(ok2, err2)) => {
                let ok = self.unify(ok1, ok2)?;
                let err = self.unify(err1, err2)?;
                Some(Type::Result(Box::new(ok), Box::new(err)))
            },
            
            // Function types unify if their parameter and return types unify
            (Type::Function(f1), Type::Function(f2)) => {
                if f1.params.len() != f2.params.len() {
                    return None;
                }
                
                let mut unified_params = Vec::new();
                for (p1, p2) in f1.params.iter().zip(f2.params.iter()) {
                    unified_params.push(self.unify(p1, p2)?);
                }
                
                let return_type = self.unify(&f1.return_type, &f2.return_type)?;
                
                Some(Type::Function(FunctionType {
                    params: unified_params,
                    return_type: Box::new(return_type),
                    type_params: vec![], // TODO: Handle type parameter unification
                }))
            },
            
            // Types that don't match any of the above patterns can't be unified
            _ => None,
        }
    }

    /// Checks if one type can be coerced into another
    pub fn can_coerce(&self, from: &Type, to: &Type) -> bool {
        // Direct equality always works
        if from == to {
            return true;
        }

        match (from, to) {
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
            
            _ => false,
        }
    }
}
