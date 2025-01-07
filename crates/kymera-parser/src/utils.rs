// Add any utility functions needed for parsing here.
// For example:

use crate::ast::AstNode;
use crate::err::{KymeraParserError, Result};
use crate::position::Span;

/// Checks if the given AST node is a valid expression.
pub fn is_valid_expression(node: &AstNode) -> bool {
    matches!(node, AstNode::Expression(_))
}

/// Checks if the given AST node is a valid statement.
pub fn is_valid_statement(node: &AstNode) -> bool {
    matches!(node, AstNode::Statement(_))
}

/// Checks if the given AST node is a valid function definition.
pub fn is_valid_function(node: &AstNode) -> bool {
    matches!(node, AstNode::Statement(crate::ast::Statement::Function(_)))
}

/// Checks if the given AST node is a valid struct definition.
pub fn is_valid_struct(node: &AstNode) -> bool {
    matches!(node, AstNode::Statement(crate::ast::Statement::Struct(_)))
}

/// Checks if the given AST node is a valid enum definition.
pub fn is_valid_enum(node: &AstNode) -> bool {
    matches!(node, AstNode::Statement(crate::ast::Statement::Enum(_)))
}

/// Validates that an expression node is of the expected type
pub fn validate_expression(node: &AstNode, expected: &str, span: Span) -> Result<()> {
    if !is_valid_expression(node) {
        return Err(KymeraParserError::Parser {
            message: format!("Expected {}, found statement", expected),
            span,
        });
    }
    Ok(())
}

/// Validates that a statement node is of the expected type
pub fn validate_statement(node: &AstNode, expected: &str, span: Span) -> Result<()> {
    if !is_valid_statement(node) {
        return Err(KymeraParserError::Parser {
            message: format!("Expected {}, found expression", expected),
            span,
        });
    }
    Ok(())
}

/// Validates that a node is a function definition
pub fn validate_function(node: &AstNode, span: Span) -> Result<()> {
    if !is_valid_function(node) {
        return Err(KymeraParserError::Parser {
            message: "Expected function definition".to_string(),
            span,
        });
    }
    Ok(())
}

/// Validates that a node is a struct definition
pub fn validate_struct(node: &AstNode, span: Span) -> Result<()> {
    if !is_valid_struct(node) {
        return Err(KymeraParserError::Parser {
            message: "Expected struct definition".to_string(),
            span,
        });
    }
    Ok(())
}

/// Validates that a node is an enum definition
pub fn validate_enum(node: &AstNode, span: Span) -> Result<()> {
    if !is_valid_enum(node) {
        return Err(KymeraParserError::Parser {
            message: "Expected enum definition".to_string(),
            span,
        });
    }
    Ok(())
}