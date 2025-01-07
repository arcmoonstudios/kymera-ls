use crate::position::Span;
use crate::lexer::TokenType;

/// Represents a literal value in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// An integer literal.
    Int(i64, Span),
    /// A float literal.
    Float(f64, Span),
    /// A boolean literal.
    Bool(bool, Span),
    /// A string literal.
    Strng(String, Span),
    /// A Stilo (string slice) literal.
    Stilo(String, Span),
    /// A nil (null) literal.
    Nil(Span),
}

/// Represents a binary operation in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    /// The left-hand side of the operation.
    pub left: Box<AstNode>,
    /// The operator.
    pub op: String,
    /// The right-hand side of the operation.
    pub right: Box<AstNode>,
    /// The location of the binary operation in the source code.
    pub span: Span,
}

/// Represents a unary operation in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOp {
    /// The operator.
    pub op: String,
    /// The operand.
    pub operand: Box<AstNode>,
    /// The location of the unary operation in the source code.
    pub span: Span,
}

/// Represents a variable declaration in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    /// The name of the variable.
    pub name: String,
    /// The value assigned to the variable.
    pub value: Literal,
    /// The location of the declaration in the source code.
    pub span: Span,
}

/// Represents a variable assignment in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    /// The name of the variable being assigned to.
    pub name: String,
    /// The new value of the variable.
    pub value: Box<AstNode>,
    /// The location of the assignment in the source code.
    pub span: Span,
}

/// Represents an if statement in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    /// The condition of the if statement.
    pub condition: Box<AstNode>,
    /// The body of the if statement.
    pub body: Vec<AstNode>,
    /// The else block of the if statement, if any.
    pub else_body: Option<Vec<AstNode>>,
    /// The location of the if statement in the source code.
    pub span: Span,
}

/// Represents a loop statement in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    /// The condition of the loop.
    pub condition: Box<AstNode>,
    /// The body of the loop.
    pub body: Vec<AstNode>,
    /// The location of the loop statement in the source code.
    pub span: Span,
}

/// Represents a return statement in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    /// The value being returned.
    pub value: Box<AstNode>,
    /// The location of the return statement in the source code.
    pub span: Span,
}

/// Represents a function call in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// The name of the function being called.
    pub name: String,
    /// The arguments passed to the function.
    pub args: Vec<AstNode>,
    /// The location of the function call in the source code.
    pub span: Span,
}

/// Represents a function definition in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The parameters of the function.
    pub params: Vec<String>,
    /// The body of the function.
    pub body: Vec<AstNode>,
    /// The location of the function definition in the source code.
    pub span: Span,
}

/// Represents a struct definition in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    /// The name of the struct.
    pub name: String,
    /// The fields of the struct.
    pub fields: Vec<(String, String)>, // (field_name, field_type)
    /// The location of the struct definition in the source code.
    pub span: Span,
}

/// Represents an enum definition in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The variants of the enum.
    pub variants: Vec<String>,
    /// The location of the enum definition in the source code.
    pub span: Span,
}

/// Represents an import statement in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    /// The type of import (Pydes or Rudes)
    pub import_type: TokenType,
    /// The path being imported
    pub path: String,
    /// Optional alias for the import
    pub alias: Option<String>,
    /// The span of the import statement
    pub span: Span,
}

/// Represents an expression in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A literal value.
    Literal(Literal),
    /// A binary operation.
    BinaryOp(BinaryOp),
    /// A unary operation.
    UnaryOp(UnaryOp),
    /// A variable identifier.
    Identifier(String, Span),
    /// A function call.
    FunctionCall(FunctionCall),
    /// A struct field access.
    FieldAccess(String, String, Span), // (struct_name, field_name, span)
    /// An array access.
    ArrayAccess(String, Box<AstNode>, Span), // (array_name, index_expr, span)
}

/// Represents a statement in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// A variable declaration.
    Declaration(Declaration),
    /// A variable assignment.
    Assignment(Assignment),
    /// An if statement.
    IfStatement(IfStatement),
    /// A loop statement.
    LoopStatement(LoopStatement),
    /// A return statement.
    ReturnStatement(ReturnStatement),
    /// A function definition.
    Function(Function),
    /// A struct definition.
    Struct(Struct),
    /// An enum definition.
    Enum(Enum),
    /// An import statement.
    Import(Import),
    /// A block of statements.
    Block(Vec<AstNode>, Span),
    /// An expression statement.
    Expression(Expression),
}

/// Represents a node in the Abstract Syntax Tree (AST).
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// An expression node.
    Expression(Expression),
    /// A statement node.
    Statement(Statement),
}