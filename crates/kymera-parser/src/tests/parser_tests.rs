#[cfg(test)]
mod parser_tests {
    use crate::ast::{AstNode, BinaryOp, Declaration, Expression, Function, IfStatement, Literal, LoopStatement, ReturnStatement, Statement, Struct, UnaryOp, Enum, Import};
    use crate::lexer::{Lexer, Token, TokenType};
    use crate::parser::Parser;
    use crate::position::{Position, Span};

    #[test]
    fn test_parse_declaration() {
        let tokens = vec![
            Token { token_type: TokenType::Djq, lexeme: "djq".to_string(), span: Span::new(Position::new(1, 1), Position::new(1, 3)) },
            Token { token_type: TokenType::Identifier, lexeme: "x".to_string(), span: Span::new(Position::new(1, 5), Position::new(1, 5)) },
            Token { token_type: TokenType::Eq, lexeme: "=".to_string(), span: Span::new(Position::new(1, 7), Position::new(1, 7)) },
            Token { token_type: TokenType::IntLiteral(10), lexeme: "10".to_string(), span: Span::new(Position::new(1, 9), Position::new(1, 10)) },
            Token { token_type: TokenType::Semicolon, lexeme: ";".to_string(), span: Span::new(Position::new(1, 11), Position::new(1, 11)) },
            Token { token_type: TokenType::Eof, lexeme: "".to_string(), span: Span::new(Position::new(1, 12), Position::new(1, 12)) },
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast[0],
            AstNode::Statement(Statement::Declaration(Declaration {
                name: "x".to_string(),
                value: Literal::Int(10, Span::new(Position::new(1, 9), Position::new(1,10))),
                span: Span::new(Position::new(1, 1), Position::new(1, 11)),
            }))
        );
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = vec![
            Token { token_type: TokenType::Identifier, lexeme: "x".to_string(), span: Span::new(Position::new(1, 1), Position::new(1, 1)) },
            Token { token_type: TokenType::Eq, lexeme: "=".to_string(), span: Span::new(Position::new(1, 3), Position::new(1, 3)) },
            Token { token_type: TokenType::IntLiteral(20), lexeme: "20".to_string(), span: Span::new(Position::new(1, 5), Position::new(1, 6)) },
            Token { token_type: TokenType::Semicolon, lexeme: ";".to_string(), span: Span::new(Position::new(1, 7), Position::new(1, 7)) },
            Token { token_type: TokenType::Eof, lexeme: "".to_string(), span: Span::new(Position::new(1, 8), Position::new(1, 8)) },
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast[0],
            AstNode::Statement(Statement::Assignment(Assignment {
                name: "x".to_string(),
                value: Box::new(AstNode::Expression(Expression::Literal(Literal::Int(
                    20,
                    Span::new(Position::new(1, 5), Position::new(1, 6))
                )))),
                span: Span::new(Position::new(1, 1), Position::new(1, 7)),
            }))
        );
    }

    #[test]
    fn test_parse_function_call() {
        let tokens = vec![
            Token { token_type: TokenType::Identifier, lexeme: "foo".to_string(), span: Span::new(Position::new(1, 1), Position::new(1, 3)) },
            Token { token_type: TokenType::LParen, lexeme: "(".to_string(), span: Span::new(Position::new(1, 4), Position::new(1, 4)) },
            Token { token_type: TokenType::IntLiteral(1), lexeme: "1".to_string(), span: Span::new(Position::new(1, 5), Position::new(1, 5)) },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), span: Span::new(Position::new(1, 6), Position::new(1, 6)) },
            Token { token_type: TokenType::IntLiteral(2), lexeme: "2".to_string(), span: Span::new(Position::new(1, 8), Position::new(1, 8)) },
            Token { token_type: TokenType::RParen, lexeme: ")".to_string(), span: Span::new(Position::new(1, 9), Position::new(1, 9)) },
            Token { token_type: TokenType::Semicolon, lexeme: ";".to_string(), span: Span::new(Position::new(1, 10), Position::new(1, 10)) },
            Token { token_type: TokenType::Eof, lexeme: "".to_string(), span: Span::new(Position::new(1, 11), Position::new(1, 11)) },
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast[0],
            AstNode::Statement(Statement::Expression(
                Expression::FunctionCall(FunctionCall {
                    name: "foo".to_string(),
                    args: vec![
                        AstNode::Expression(Expression::Literal(Literal::Int(
                            1,
                            Span::new(Position::new(1, 5), Position::new(1, 5))
                        ))),
                        AstNode::Expression(Expression::Literal(Literal::Int(
                            2,
                            Span::new(Position::new(1, 8), Position::new(1, 8))
                        ))),
                    ],
                    span: Span::new(Position::new(1, 1), Position::new(1, 9)),
                })
            ))
        );
    }

    #[test]
    fn test_parse_python_import() {
        let tokens = vec![
            Token {
                token_type: TokenType::Pydes,
                lexeme: "pydes".to_string(),
                span: Span::new(Position::new(1, 1), Position::new(1, 5)),
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "numpy".to_string(),
                span: Span::new(Position::new(1, 7), Position::new(1, 11)),
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                span: Span::new(Position::new(1, 12), Position::new(1, 12)),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span::new(Position::new(1, 13), Position::new(1, 13)),
            },
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast[0],
            AstNode::Statement(Statement::Import(Import {
                import_type: TokenType::Pydes,
                path: "numpy".to_string(),
                alias: None,
                span: Span::new(Position::new(1, 1), Position::new(1, 12)),
            }))
        );
    }

    #[test]
    fn test_parse_rust_import() {
        let tokens = vec![
            Token {
                token_type: TokenType::Rudes,
                lexeme: "rudes".to_string(),
                span: Span::new(Position::new(1, 1), Position::new(1, 5)),
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "serde".to_string(),
                span: Span::new(Position::new(1, 7), Position::new(1, 11)),
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                span: Span::new(Position::new(1, 12), Position::new(1, 12)),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span::new(Position::new(1, 13), Position::new(1, 13)),
            },
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast[0],
            AstNode::Statement(Statement::Import(Import {
                import_type: TokenType::Rudes,
                path: "serde".to_string(),
                alias: None,
                span: Span::new(Position::new(1, 1), Position::new(1, 12)),
            }))
        );
    }

    #[test]
    fn test_parse_python_import_with_alias() {
        let tokens = vec![
            Token {
                token_type: TokenType::Pydes,
                lexeme: "pydes".to_string(),
                span: Span::new(Position::new(1, 1), Position::new(1, 5)),
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "numpy".to_string(),
                span: Span::new(Position::new(1, 7), Position::new(1, 11)),
            },
            Token {
                token_type: TokenType::As,
                lexeme: "as".to_string(),
                span: Span::new(Position::new(1, 13), Position::new(1, 14)),
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "np".to_string(),
                span: Span::new(Position::new(1, 16), Position::new(1, 17)),
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                span: Span::new(Position::new(1, 18), Position::new(1, 18)),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span::new(Position::new(1, 19), Position::new(1, 19)),
            },
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast[0],
            AstNode::Statement(Statement::Import(Import {
                import_type: TokenType::Pydes,
                path: "numpy".to_string(),
                alias: Some("np".to_string()),
                span: Span::new(Position::new(1, 1), Position::new(1, 18)),
            }))
        );
    }

    // ... Add more parser tests for different language constructs ...
}