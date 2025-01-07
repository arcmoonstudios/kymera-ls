use crate::err::{ParserError, Result};
use crate::lexer::{Token, TokenType};
use crate::position::{Position, Span};
use crate::ast::{AstNode, BinaryOp, Declaration, Expression, Function, IfStatement, 
    Literal, LoopStatement, ReturnStatement, Statement, Struct, UnaryOp, Enum, Import, FunctionCall, Assignment};
use tracing::debug;

/// Parser for the Kymera language.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser for the given tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses the tokens and returns a vector of AST nodes.
    pub fn parse(&mut self) -> Result<Vec<AstNode>> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            nodes.push(self.parse_statement()?);
        }

        debug!("Parsed AST: {:?}", nodes);
        Ok(nodes)
    }

    /// Parses a statement.
    fn parse_statement(&mut self) -> Result<AstNode> {
        match self.peek()?.token_type {
            TokenType::Pydes | TokenType::Rudes => {
                let import = self.parse_import()?;
                Ok(AstNode::Statement(Statement::Import(import)))
            }
            TokenType::Fnc => self.parse_function(),
            TokenType::Forma => self.parse_struct(),
            TokenType::Enum => self.parse_enum(),
            TokenType::Ret => self.parse_return_statement(),
            TokenType::Wyo => self.parse_loop_statement(),
            TokenType::Ate => self.parse_if_statement(),
            TokenType::Djq => self.parse_declaration(),
            TokenType::Idit => {
                let next_token = self.peek_next()?;
                match next_token.token_type {
                    TokenType::Eq => self.parse_assignment(),
                    _ => self.parse_expression_statement(),
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    /// Parses an import statement.
    fn parse_import(&mut self) -> Result<Import> {
        let start_pos = self.current_token()?.span.start;
        let import_type = self.current_token()?.token_type.clone();

        // Consume the import keyword (pydes or rudes)
        self.advance();

        // Parse the import path
        let path = match &self.current_token()?.token_type {
            TokenType::Identifier(_) => {
                let path = self.current_token()?.lexeme.clone();
                self.advance();
                path
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.current_token()?.lexeme.clone(),
                    span: self.current_token()?.span,
                });
            }
        };

        // Check for optional alias
        let alias = if let TokenType::As = self.current_token()?.token_type {
            self.advance(); // Consume 'as'
            match &self.current_token()?.token_type {
                TokenType::Identifier(_) => {
                    let alias = self.current_token()?.lexeme.clone();
                    self.advance();
                    Some(alias)
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: self.current_token()?.lexeme.clone(),
                        span: self.current_token()?.span,
                    });
                }
            }
        } else {
            None
        };

        // Expect semicolon
        if self.current_token()?.token_type != TokenType::Semicolon {
            return Err(ParserError::UnexpectedToken {
                expected: ";".to_string(),
                found: self.current_token()?.lexeme.clone(),
                span: self.current_token()?.span,
            });
        }
        let end_pos = self.current_token()?.span.end;
        self.advance(); // Consume semicolon

        Ok(Import {
            import_type,
            path,
            alias,
            span: Span::new(start_pos, end_pos),
        })
    }

    // Parses a struct definition
    fn parse_struct(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Des)?; // Consume 'des'
        let name_token = self.consume(TokenType::Identifier(String::new()))?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LBrace)?; // Consume '{'

        let mut fields = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let field_name_token = self.consume(TokenType::Identifier(String::new()))?;
            let field_name = field_name_token.lexeme.clone();

            self.consume(TokenType::Colon)?; // Consume ':'

            let field_type_token = self.consume(TokenType::Identifier(String::new()))?;
            let field_type = field_type_token.lexeme.clone();

            fields.push((field_name, field_type));

            if !self.match_token(TokenType::Comma) {
                break;
            }
        }

        self.consume(TokenType::RBrace)?; // Consume '}'
        let end_pos = self.previous_token()?.span.end;

        Ok(AstNode::Statement(Statement::Struct(Struct {
            name,
            fields,
            span: Span::new(start_pos, end_pos),
        })))
    }

    // Parses an enum definition
    fn parse_enum(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Enum)?; // Consume 'enum'
        let name_token = self.consume(TokenType::Identifier(String::new()))?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LBrace)?; // Consume '{'

        let mut variants = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let variant_name_token = self.consume(TokenType::Identifier(String::new()))?;
            let variant_name = variant_name_token.lexeme.clone();
            variants.push(variant_name);

            if !self.match_token(TokenType::Comma) {
                break;
            }
        }

        self.consume(TokenType::RBrace)?; // Consume '}'
        let end_pos = self.previous_token()?.span.end;

        Ok(AstNode::Statement(Statement::Enum(Enum {
            name,
            variants,
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses a function definition.
    fn parse_function(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Fnc)?; // Consume 'fnc'
        let name_token = self.consume(TokenType::Identifier(String::new()))?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LParen)?; // Consume '('
        let mut params = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                let param_token = self.consume(TokenType::Identifier(String::new()))?;
                params.push(param_token.lexeme.clone());
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RParen)?; // Consume ')'

        let body = self.parse_block_statement()?;
        let end_pos = self.previous_token()?.span.end;

        Ok(AstNode::Statement(Statement::Function(Function {
            name,
            params,
            body,
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses a return statement.
    fn parse_return_statement(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Ret)?; // Consume 'ret'
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?; // Consume ';'
        let end_pos = self.previous_token()?.span.end;
        Ok(AstNode::Statement(Statement::ReturnStatement(ReturnStatement {
            value: Box::new(value),
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses an if statement.
    fn parse_if_statement(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Ate)?; // Consume 'ate'
        let condition = self.parse_expression()?;
        let body = self.parse_block_statement()?;
        let else_body = if self.match_token(TokenType::Rev) {
            Some(self.parse_block_statement()?)
        } else {
            None
        };
        let end_pos = self.previous_token()?.span.end;
        Ok(AstNode::Statement(Statement::IfStatement(IfStatement {
            condition: Box::new(condition),
            body,
            else_body,
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses a loop statement.
    fn parse_loop_statement(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Wyo)?; // Consume 'wyo'
        let condition = self.parse_expression()?;
        let body = self.parse_block_statement()?;
        let end_pos = self.previous_token()?.span.end;
        Ok(AstNode::Statement(Statement::LoopStatement(LoopStatement {
            condition: Box::new(condition),
            body,
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses a block statement.
    fn parse_block_statement(&mut self) -> Result<Vec<AstNode>> {
        self.consume(TokenType::LBrace)?; // Consume '{'
        let mut statements = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        self.consume(TokenType::RBrace)?; // Consume '}'
        Ok(statements)
    }

    /// Parses a declaration statement.
    fn parse_declaration(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        self.consume(TokenType::Djq)?; // Consume 'djq'
        let name_token = self.consume(TokenType::Identifier(String::new()))?;
        let name = name_token.lexeme.clone();
        self.consume(TokenType::Eq)?; // Consume '='
        let value = self.parse_literal()?;
        self.consume(TokenType::Semicolon)?; // Consume ';'
        let end_pos = self.previous_token()?.span.end;
        Ok(AstNode::Statement(Statement::Declaration(Declaration {
            name,
            value,
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses an assignment statement.
    fn parse_assignment(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        let name_token = self.consume(TokenType::Identifier(String::new()))?;
        let name = name_token.lexeme.clone();
        self.consume(TokenType::Eq)?; // Consume '='
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?; // Consume ';'
        let end_pos = self.previous_token()?.span.end;
        Ok(AstNode::Statement(Statement::Assignment(Assignment {
            name,
            value: Box::new(value),
            span: Span::new(start_pos, end_pos),
        })))
    }

    /// Parses an expression.
    fn parse_expression(&mut self) -> Result<AstNode> {
        self.parse_assignment_expression()
    }

    /// Parses an assignment expression.
    fn parse_assignment_expression(&mut self) -> Result<AstNode> {
        let left = self.parse_or_expression()?;
        if self.match_token(TokenType::Eq) {
            let start_pos = self.current_token()?.span.start;
            let right = self.parse_assignment_expression()?;
            let end_pos = self.previous_token()?.span.end;
            if let AstNode::Expression(Expression::Identifier(name, _)) = left {
                Ok(AstNode::Statement(Statement::Assignment(Assignment {
                    name,
                    value: Box::new(right),
                    span: Span::new(start_pos, end_pos),
                })))
            } else {
                Err(ParserError::Parser {
                    message: "Invalid assignment target".to_string(),
                    span: Span::new(start_pos, end_pos),
                })
            }
        } else {
            Ok(left)
        }
    }

    // Parses an 'or' expression.
    fn parse_or_expression(&mut self) -> Result<AstNode> {
        let mut left = self.parse_and_expression()?;
        while self.match_token(TokenType::Or) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let right = self.parse_and_expression()?;
            let end_pos = self.previous_token()?.span.end;
            left = AstNode::Expression(Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::new(start_pos, end_pos),
            }));
        }
        Ok(left)
    }

    // Parses an 'and' expression.
    fn parse_and_expression(&mut self) -> Result<AstNode> {
        let mut left = self.parse_equality_expression()?;
        while self.match_token(TokenType::And) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let right = self.parse_equality_expression()?;
            let end_pos = self.previous_token()?.span.end;
            left = AstNode::Expression(Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::new(start_pos, end_pos),
            }));
        }
        Ok(left)
    }

    // Parses an equality expression.
    fn parse_equality_expression(&mut self) -> Result<AstNode> {
        let mut left = self.parse_comparison_expression()?;
        while self.match_tokens(&[TokenType::EqEq, TokenType::Ne]) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let right = self.parse_comparison_expression()?;
            let end_pos = self.previous_token()?.span.end;
            left = AstNode::Expression(Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::new(start_pos, end_pos),
            }));
        }
        Ok(left)
    }

    // Parses a comparison expression.
    fn parse_comparison_expression(&mut self) -> Result<AstNode> {
        let mut left = self.parse_term()?;
        while self.match_tokens(&[TokenType::Gt, TokenType::Lt, TokenType::Ge, TokenType::Le]) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let right = self.parse_term()?;
            let end_pos = self.previous_token()?.span.end;
            left = AstNode::Expression(Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::new(start_pos, end_pos),
            }));
        }
        Ok(left)
    }

    // Parses a term expression.
    fn parse_term(&mut self) -> Result<AstNode> {
        let mut left = self.parse_factor()?;
        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let right = self.parse_factor()?;
            let end_pos = self.previous_token()?.span.end;
            left = AstNode::Expression(Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::new(start_pos, end_pos),
            }));
        }
        Ok(left)
    }

    /// Parses a factor expression.
    fn parse_factor(&mut self) -> Result<AstNode> {
        let mut left = self.parse_unary()?;
        while self.match_tokens(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let right = self.parse_unary()?;
            let end_pos = self.previous_token()?.span.end;
            left = AstNode::Expression(Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::new(start_pos, end_pos),
            }));
        }
        Ok(left)
    }

    /// Parses a unary expression.
    fn parse_unary(&mut self) -> Result<AstNode> {
        if self.match_tokens(&[TokenType::Minus, TokenType::Not]) {
            let start_pos = self.current_token()?.span.start;
            let op = self.previous_token()?.lexeme.clone();
            let operand = self.parse_unary()?;
            let end_pos = self.previous_token()?.span.end;
            Ok(AstNode::Expression(Expression::UnaryOp(UnaryOp {
                op,
                operand: Box::new(operand),
                span: Span::new(start_pos, end_pos),
            })))
        } else {
            self.parse_primary()
        }
    }

    /// Parses a primary expression.
    fn parse_primary(&mut self) -> Result<AstNode> {
        let token = self.current_token()?;
        match token.token_type {
            TokenType::IntLiteral(val) => {
                self.advance();
                Ok(AstNode::Expression(Expression::Literal(Literal::Int(
                    val,
                    token.span,
                ))))
            }
            TokenType::FloatLiteral(val) => {
                self.advance();
                Ok(AstNode::Expression(Expression::Literal(Literal::Float(
                    val,
                    token.span,
                ))))
            }
            TokenType::StringLiteral(val) => {
                self.advance();
                Ok(AstNode::Expression(Expression::Literal(Literal::Strng(
                    val,
                    token.span,
                ))))
            }
            TokenType::BoolLiteral(val) => {
                self.advance();
                Ok(AstNode::Expression(Expression::Literal(Literal::Bool(
                    val,
                    token.span,
                ))))
            }
            TokenType::Nil => {
                self.advance();
                Ok(AstNode::Expression(Expression::Literal(Literal::Nil(
                    token.span,
                ))))
            }
            TokenType::Identifier(_) => self.parse_identifier_expression(),
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RParen)?;
                Ok(expr)
            }
            _ => Err(self.error("Expected literal, identifier, or '('")),
        }
    }

    /// Parses an identifier-based expression (variable, function call, etc.).
    fn parse_identifier_expression(&mut self) -> Result<AstNode> {
        let start_pos = self.current_token()?.span.start;
        let name_token = self.consume(TokenType::Identifier(String::new()))?;
        let name = name_token.lexeme.clone();

        if self.match_token(TokenType::LParen) {
            let args = self.parse_function_call_arguments()?;
            let end_pos = self.previous_token()?.span.end;
            Ok(AstNode::Expression(Expression::FunctionCall(FunctionCall {
                name,
                args,
                span: Span::new(start_pos, end_pos),
            })))
        } else {
            let end_pos = self.previous_token()?.span.end;
            Ok(AstNode::Expression(Expression::Identifier(
                name,
                Span::new(start_pos, end_pos),
            )))
        }
    }

    /// Parses the arguments of a function call.
    fn parse_function_call_arguments(&mut self) -> Result<Vec<AstNode>> {
        let mut args = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                let arg = self.parse_expression()?;
                args.push(arg);
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RParen)?;
        Ok(args)
    }

    /// Parses a literal value.
    fn parse_literal(&mut self) -> Result<Literal> {
        let token = self.current_token()?;
        match token.token_type {
            TokenType::IntLiteral(val) => {
                self.advance();
                Ok(Literal::Int(val, token.span))
            }
            TokenType::FloatLiteral(val) => {
                self.advance();
                Ok(Literal::Float(val, token.span))
            }
            TokenType::StringLiteral(val) => {
                self.advance();
                Ok(Literal::Strng(val, token.span))
            }
            TokenType::BoolLiteral(val) => {
                self.advance();
                Ok(Literal::Bool(val, token.span))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Literal::Nil(token.span))
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: "literal".to_string(),
                found: token.lexeme.clone(),
                span: token.span,
            }),
        }
    }

    /// Parses an expression statement.
    fn parse_expression_statement(&mut self) -> Result<AstNode> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?; // Consume ';'
        match expr {
            AstNode::Expression(e) => Ok(AstNode::Statement(Statement::Expression(e))),
            _ => Err(self.error("Expected expression")),
        }
    }

    /// Consumes the current token if it matches the expected type.
    fn consume(&mut self, expected_type: TokenType) -> Result<Token> {
        let token = self.current_token()?;
        if token.token_type == expected_type {
            self.advance();
            Ok(token)
        } else {
            let span = token.span;
            Err(ParserError::UnexpectedToken {
                expected: format!("{:?}", expected_type),
                found: token.lexeme.clone(),
                span,
            })
        }
    }

    /// Checks if the current token matches the given type without consuming it.
    fn check(&mut self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.current_token().map_or(false, |t| t.token_type == token_type)
    }

    /// Returns the current token without consuming it.
    fn current_token(&self) -> Result<Token> {
        if self.current >= self.tokens.len() {
            Err(ParserError::UnexpectedEof {
                span: Span::new(Position::new(0, 0, 0), Position::new(0, 0, 0)),
            })
        } else {
            Ok(self.tokens[self.current].clone())
        }
    }

    /// Returns the next token without consuming it.
    fn peek(&self) -> Result<Token> {
        if self.current >= self.tokens.len() {
            Err(ParserError::UnexpectedEof {
                span: Span::new(Position::new(0, 0, 0), Position::new(0, 0, 0)),
            })
        } else {
            Ok(self.tokens[self.current].clone())
        }
    }

    /// Returns the token after the next without consuming it.
    fn peek_next(&self) -> Result<Token> {
        if self.current + 1 >= self.tokens.len() {
            Err(ParserError::UnexpectedEof {
                span: Span::new(Position::new(0, 0, 0), Position::new(0, 0, 0)),
            })
        } else {
            Ok(self.tokens[self.current + 1].clone())
        }
    }

    /// Returns the previously consumed token.
    fn previous_token(&self) -> Result<Token> {
        if self.current == 0 {
            Err(ParserError::UnexpectedEof {
                span: Span::new(Position::new(0, 0, 0), Position::new(0, 0, 0)),
            })
        } else {
            Ok(self.tokens[self.current - 1].clone())
        }
    }

    /// Advances to the next token.
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    /// Checks if the current token is the end of input.
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].token_type == TokenType::Eof
    }

    /// Consumes the current token if its type matches any of the given types.
    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(ty.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Consumes the current token if its type matches the given type.
    fn match_token(&mut self, ty: TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn error(&self, message: impl Into<String>) -> ParserError {
        let span = self.current_token()
            .map(|t| t.span)
            .unwrap_or_else(|_| Span::new(Position::new(0, 0, 0), Position::new(0, 0, 0)));
        ParserError::Parser {
            message: message.into(),
            span,
        }
    }
}