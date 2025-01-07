//! Core types for the Kymera reactor system.
//!
//! Defines the foundational types that represent:
//! - Language constructs (modules, structures, implementations)
//! - Neural analysis and optimization structures
//! - Memory and resource management types
//! - Type system representations
//! - Error handling and results

use std::{fmt::Debug, sync::Arc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Core result type for reactor operations
pub type ReactorResult<T> = Result<T, ReactorError>;

/// Result type for module-level operations
pub type ModuleResult<T> = Result<T, ModuleError>;

/// Neural analysis output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralAnalysis {
    /// Analyzed code structure
    pub structure: CodeStructure,
    /// Identified patterns
    pub patterns: Vec<Pattern>,
    /// Optimization suggestions
    pub optimizations: Vec<Optimization>,
    /// Performance metrics
    pub metrics: AnalysisMetrics,
}

/// Code structure analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStructure {
    /// Module dependencies
    pub dependencies: Vec<Dependency>,
    /// Type relationships
    pub type_relations: Vec<TypeRelation>,
    /// Control flow graph
    pub control_flow: ControlFlow,
    /// Memory usage patterns
    pub memory_patterns: Vec<MemoryPattern>,
}

/// Pattern identified in code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern type
    pub pattern_type: PatternType,
    /// Pattern location
    pub location: Location,
    /// Pattern confidence score
    pub confidence: f64,
}

/// Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optimization {
    /// Optimization type
    pub opt_type: OptimizationType,
    /// Expected improvement
    pub improvement: Improvement,
    /// Implementation complexity
    pub complexity: Complexity,
}

/// Analysis metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetrics {
    /// Analysis duration
    pub duration: std::time::Duration,
    /// Memory usage
    pub memory_usage: usize,
    /// GPU utilization
    pub gpu_utilization: f64,
}

/// Code reasoning output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReasoning {
    /// Reasoning steps
    pub steps: Vec<ReasoningStep>,
    /// Conclusions
    pub conclusions: Vec<Conclusion>,
    /// Confidence scores
    pub confidence: HashMap<String, f64>,
}

/// Optimized code output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedCode {
    /// Original code
    pub original: String,
    /// Optimized version
    pub optimized: String,
    /// Applied optimizations
    pub optimizations: Vec<AppliedOptimization>,
    /// Performance metrics
    pub metrics: OptimizationMetrics,
}

/// Module representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Module path
    pub path: String,
    /// Module exports
    pub exports: Vec<Export>,
    /// Module dependencies
    pub dependencies: Vec<Dependency>,
    /// Module documentation
    pub documentation: Documentation,
}

/// Structure definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    /// Structure name
    pub name: String,
    /// Structure fields
    pub fields: Vec<Field>,
    /// Structure attributes
    pub attributes: Vec<Attribute>,
    /// Memory layout
    pub layout: MemoryLayout,
    /// Generic parameters
    pub generics: Vec<GenericParam>,
    /// Associated types
    pub associated_types: Vec<AssociatedType>,
    /// Default implementations
    pub defaults: Vec<DefaultImpl>,
    /// Documentation
    pub documentation: Documentation,
}

/// Generic parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericParam {
    /// Parameter name
    pub name: String,
    /// Parameter bounds
    pub bounds: Vec<String>,
    /// Default type
    pub default: Option<String>,
}

/// Associated type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociatedType {
    /// Type name
    pub name: String,
    /// Type bounds
    pub bounds: Vec<String>,
    /// Default type
    pub default: Option<String>,
}

/// Default implementation for a trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultImpl {
    /// Target trait
    pub trait_name: String,
    /// Implementation methods
    pub methods: Vec<Method>,
}

/// Implementation block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
    /// Target type
    pub target_type: String,
    /// Implemented methods
    pub methods: Vec<Method>,
    /// Implementation attributes
    pub attributes: Vec<Attribute>,
    /// Generic parameters
    pub generics: Vec<GenericParam>,
    /// Associated types
    pub associated_types: Vec<AssociatedType>,
    /// Trait bounds
    pub trait_bounds: Vec<String>,
    /// Where clauses
    pub where_clauses: Vec<WhereClause>,
    /// Documentation
    pub documentation: Documentation,
}

/// Where clause definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereClause {
    /// Type parameter
    pub type_param: String,
    /// Bounds
    pub bounds: Vec<String>,
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Function parameters
    pub params: Vec<Parameter>,
    /// Return type
    pub return_type: Type,
    /// Function body
    pub body: Vec<Statement>,
    /// Function attributes
    pub attributes: Vec<Attribute>,
    /// Generic parameters
    pub generics: Vec<GenericParam>,
    /// Where clauses
    pub where_clauses: Vec<WhereClause>,
    /// Async marker
    pub is_async: bool,
    /// Const marker
    pub is_const: bool,
    /// Unsafe marker
    pub is_unsafe: bool,
    /// Documentation
    pub documentation: Documentation,
    /// Error handling
    pub error_handling: ErrorHandling,
}

/// Error handling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandling {
    /// Error type
    pub error_type: Option<Type>,
    /// Recovery strategy
    pub recovery: RecoveryStrategy,
    /// Custom error handlers
    pub handlers: Vec<ErrorHandler>,
}

/// Error recovery strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Propagate the error
    Propagate,
    /// Return a default value
    ReturnDefault,
    /// Custom recovery function
    Custom(String),
}

/// Error handler definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandler {
    /// Error type to handle
    pub error_type: Type,
    /// Handler function
    pub handler: String,
}

/// Method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    /// Method name
    pub name: String,
    /// Method parameters
    pub params: Vec<Parameter>,
    /// Return type
    pub return_type: Type,
    /// Method body
    pub body: Vec<Statement>,
    /// Method attributes
    pub attributes: Vec<Attribute>,
}

/// Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub type_: Type,
    /// Parameter attributes
    pub attributes: Vec<Attribute>,
}

/// Type representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    /// Integer types
    Int(IntSize),
    /// Floating point types
    Float(FloatSize),
    /// String type
    String,
    /// Boolean type
    Bool,
    /// Custom type
    Custom(String),
    /// Generic type
    Generic(String, Vec<Type>),
}

/// Integer size variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

/// Float size variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FloatSize {
    F32,
    F64,
}

/// Statement representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// Expression statement
    Expression(Expression),
    /// Let binding
    Let(String, Type, Expression),
    /// Return statement
    Return(Option<Expression>),
    /// If statement
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    /// Loop statement
    Loop(Vec<Statement>),
}

/// Expression representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// Literal value
    Literal(Literal),
    /// Variable reference
    Variable(String),
    /// Function call
    Call(String, Vec<Expression>),
    /// Method call
    MethodCall(Box<Expression>, String, Vec<Expression>),
    /// Binary operation
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
}

/// Literal value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// Integer literal
    Int(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Bool(bool),
}

/// Binary operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Memory layout information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLayout {
    /// Total size in bytes
    pub size: usize,
    /// Alignment in bytes
    pub alignment: usize,
    /// Field offsets
    pub field_offsets: HashMap<String, usize>,
}

/// Documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    /// Documentation text
    pub text: String,
    /// Examples
    pub examples: Vec<Example>,
    /// See also references
    pub see_also: Vec<String>,
}

/// Code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    /// Example title
    pub title: String,
    /// Example code
    pub code: String,
    /// Example output
    pub output: Option<String>,
}

/// Export definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    /// Export name
    pub name: String,
    /// Export kind
    pub kind: ExportKind,
    /// Export visibility
    pub visibility: Visibility,
}

/// Export kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportKind {
    Type,
    Function,
    Module,
    Constant,
}

/// Visibility level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    /// Dependency version
    pub version: String,
    /// Dependency features
    pub features: Vec<String>,
}

/// Type relation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRelation {
    /// Source type
    pub source: String,
    /// Target type
    pub target: String,
    /// Relation kind
    pub kind: RelationKind,
}

/// Relation kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationKind {
    Implements,
    Extends,
    Uses,
    Contains,
}

/// Control flow representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFlow {
    /// Flow nodes
    pub nodes: Vec<FlowNode>,
    /// Flow edges
    pub edges: Vec<FlowEdge>,
}

/// Flow node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    /// Node ID
    pub id: usize,
    /// Node kind
    pub kind: FlowNodeKind,
}

/// Flow node kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowNodeKind {
    Entry,
    Exit,
    Basic,
    Branch,
    Loop,
}

/// Flow edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    /// Source node ID
    pub source: usize,
    /// Target node ID
    pub target: usize,
    /// Edge kind
    pub kind: FlowEdgeKind,
}

/// Flow edge kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowEdgeKind {
    Normal,
    True,
    False,
    Continue,
    Break,
}

/// Memory pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPattern {
    /// Pattern kind
    pub kind: MemoryPatternKind,
    /// Pattern location
    pub location: Location,
    /// Pattern impact
    pub impact: Impact,
}

/// Memory pattern kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryPatternKind {
    Allocation,
    Deallocation,
    Leak,
    UseAfterFree,
    DoubleFree,
}

/// Code location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// File path
    pub file: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
}

/// Pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Performance,
    Security,
    Style,
    Bug,
}

/// Optimization type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    Inlining,
    Vectorization,
    LoopUnrolling,
    MemoryCoalescing,
}

/// Expected improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Improvement {
    /// Speed improvement
    pub speed: f64,
    /// Memory improvement
    pub memory: f64,
}

/// Implementation complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Complexity {
    Low,
    Medium,
    High,
}

/// Reasoning step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step description
    pub description: String,
    /// Step confidence
    pub confidence: f64,
}

/// Reasoning conclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conclusion {
    /// Conclusion type
    pub conclusion_type: ConclusionType,
    /// Conclusion details
    pub details: String,
}

/// Conclusion type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConclusionType {
    Optimization,
    Security,
    Correctness,
}

/// Applied optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedOptimization {
    /// Optimization type
    pub opt_type: OptimizationType,
    /// Location applied
    pub location: Location,
    /// Improvement achieved
    pub improvement: Improvement,
}

/// Optimization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetrics {
    /// Size reduction
    pub size_reduction: f64,
    /// Speed improvement
    pub speed_improvement: f64,
    /// Memory reduction
    pub memory_reduction: f64,
}

/// Impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Impact {
    Low,
    Medium,
    High,
    Critical,
}

/// Field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub type_: Type,
    /// Field attributes
    pub attributes: Vec<Attribute>,
}

/// Attribute definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute name
    pub name: String,
    /// Attribute arguments
    pub args: Vec<AttributeArg>,
}

/// Attribute argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeArg {
    /// String argument
    String(String),
    /// Integer argument
    Int(i64),
    /// Boolean argument
    Bool(bool),
}

use std::collections::HashMap; 