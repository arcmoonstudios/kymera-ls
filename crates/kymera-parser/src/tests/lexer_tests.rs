#[cfg(test)]
mod lexer_tests {
    use crate::lexer::{Lexer, Token, TokenType};
    use crate::position::{Position, Span};

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::new("  \t \n\r ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("|> This is a comment <| |> Another comment");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }
    #[test]
    fn test_invalid_sn() {
        let mut lexer = Lexer::new("sn");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_xn() {
        let mut lexer = Lexer::new("xn");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_w() {
        let mut lexer = Lexer::new("w");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_r() {
        let mut lexer = Lexer::new("r");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_m() {
        let mut lexer = Lexer::new("m");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_is() {
        let mut lexer = Lexer::new("is");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_us() {
        let mut lexer = Lexer::new("us");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_invalid_escape_sequence() {
        let mut lexer = Lexer::new("\"\\q\"");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_unterminated_escape_sequence() {
        let mut lexer = Lexer::new("\"\\");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_unterminated_string() {
        let mut lexer = Lexer::new("\"hello");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_unterminated_block_comment() {
        let mut lexer = Lexer::new("|> This is a comment");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_unexpected_character() {
        let mut lexer = Lexer::new("§");
        let err = lexer.tokenize().unwrap_err();
        assert!(matches!(
            err,
            crate::error::KymeraParserError::LexerError(_, _)
        ));
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("des forma enum imp fnc Res djq ret rev");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::Des);
        assert_eq!(tokens[1].token_type, TokenType::Forma);
        assert_eq!(tokens[2].token_type, TokenType::Enum);
        assert_eq!(tokens[3].token_type, TokenType::Imp);
        assert_eq!(tokens[4].token_type, TokenType::Fnc);
        assert_eq!(tokens[5].token_type, TokenType::Res);
        assert_eq!(tokens[6].token_type, TokenType::Djq);
        assert_eq!(tokens[7].token_type, TokenType::Ret);
        assert_eq!(tokens[8].token_type, TokenType::Rev);
        assert_eq!(tokens[9].token_type, TokenType::Eof);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("des forma enum imp fnc soy sn> xn> w>? Res djq ret r? wyo ate m> spa Optn Stilo Strng ~ & [=-] i8 i16 i32 i64 i128 is# u8 u16 u32 u64 u128 us# f32 f64 Prnt");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 41);
        assert_eq!(tokens[0].token_type, TokenType::Des);
        assert_eq!(tokens[1].token_type, TokenType::Forma);
        assert_eq!(tokens[2].token_type, TokenType::Enum);
        assert_eq!(tokens[3].token_type, TokenType::Imp);
        assert_eq!(tokens[4].token_type, TokenType::Fnc);
        assert_eq!(tokens[5].token_type, TokenType::Soy);
        assert_eq!(tokens[6].token_type, TokenType::Snc);
        assert_eq!(tokens[7].token_type, TokenType::Xnc);
        assert_eq!(tokens[8].token_type, TokenType::Spro);
        assert_eq!(tokens[9].token_type, TokenType::Res);
        assert_eq!(tokens[10].token_type, TokenType::Djq);
        assert_eq!(tokens[11].token_type, TokenType::Ret);
        assert_eq!(tokens[12].token_type, TokenType::Rev);
        assert_eq!(tokens[13].token_type, TokenType::Wyo);
        assert_eq!(tokens[14].token_type, TokenType::Ate);
        assert_eq!(tokens[15].token_type, TokenType::Mth);
        assert_eq!(tokens[16].token_type, TokenType::Spa);
        assert_eq!(tokens[17].token_type, TokenType::Optn);
        assert_eq!(tokens[18].token_type, TokenType::Stilo);
        assert_eq!(tokens[19].token_type, TokenType::Strng);
        assert_eq!(tokens[20].token_type, TokenType::Muta);
        assert_eq!(tokens[21].token_type, TokenType::Nmut);
        assert_eq!(tokens[22].token_type, TokenType::Ifz);
        assert_eq!(tokens[23].token_type, TokenType::I8);
        assert_eq!(tokens[24].token_type, TokenType::I16);
        assert_eq!(tokens[25].token_type, TokenType::I32);
        assert_eq!(tokens[26].token_type, TokenType::I64);
        assert_eq!(tokens[27].token_type, TokenType::I128);
        assert_eq!(tokens[28].token_type, TokenType::Isz);
        assert_eq!(tokens[29].token_type, TokenType::U8);
        assert_eq!(tokens[30].token_type, TokenType::U16);
        assert_eq!(tokens[31].token_type, TokenType::U32);
        assert_eq!(tokens[32].token_type, TokenType::U64);
        assert_eq!(tokens[33].token_type, TokenType::U128);
        assert_eq!(tokens[34].token_type, TokenType::Usz);
        assert_eq!(tokens[35].token_type, TokenType::F32);
        assert_eq!(tokens[36].token_type, TokenType::F64);
        assert_eq!(tokens[37].token_type, TokenType::Prnt);
        assert_eq!(tokens[40].token_type, TokenType::Eof);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new(":+ :> + - * / % += -= *= /= %= = == != < > <= >= && || !");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 23);
        assert_eq!(tokens[0].token_type, TokenType::Colon);
        assert_eq!(tokens[1].token_type, TokenType::Spacs);
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[3].token_type, TokenType::Minus);
        assert_eq!(tokens[4].token_type, TokenType::Star);
        assert_eq!(tokens[5].token_type, TokenType::Slash);
        assert_eq!(tokens[6].token_type, TokenType::Percent);
        assert_eq!(tokens[7].token_type, TokenType::PlusEq);
        assert_eq!(tokens[8].token_type, TokenType::MinusEq);
        assert_eq!(tokens[9].token_type, TokenType::StarEq);
        assert_eq!(tokens[10].token_type, TokenType::SlashEq);
        assert_eq!(tokens[11].token_type, TokenType::PercentEq);
        assert_eq!(tokens[12].token_type, TokenType::Eq);
        assert_eq!(tokens[13].token_type, TokenType::EqEq);
        assert_eq!(tokens[14].token_type, TokenType::Ne);
        assert_eq!(tokens[15].token_type, TokenType::Lt);
        assert_eq!(tokens[16].token_type, TokenType::Gt);
        assert_eq!(tokens[17].token_type, TokenType::Le);
        assert_eq!(tokens[18].token_type, TokenType::Ge);
        assert_eq!(tokens[19].token_type, TokenType::And);
        assert_eq!(tokens[20].token_type, TokenType::Or);
        assert_eq!(tokens[21].token_type, TokenType::Not);
        assert_eq!(tokens[22].token_type, TokenType::Eof);
    }

    #[test]
    fn test_delimiters() {
        let mut lexer = Lexer::new("(){}[],.;");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::LParen);
        assert_eq!(tokens[1].token_type, TokenType::RParen);
        assert_eq!(tokens[2].token_type, TokenType::LBrace);
        assert_eq!(tokens[3].token_type, TokenType::RBrace);
        assert_eq!(tokens[4].token_type, TokenType::LBracket);
        assert_eq!(tokens[5].token_type, TokenType::RBracket);
        assert_eq!(tokens[6].token_type, TokenType::Comma);
        assert_eq!(tokens[7].token_type, TokenType::Dot);
        assert_eq!(tokens[8].token_type, TokenType::Semicolon);
        assert_eq!(tokens[9].token_type, TokenType::Eof);
    }

    #[test]
    fn test_number_literals() {
        let mut lexer = Lexer::new("123 456.789");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::IntLiteral(123));
        assert_eq!(tokens[1].token_type, TokenType::FloatLiteral(456.789));
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"hello\" \"world\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0].token_type,
            TokenType::StringLiteral("hello".to_string())
        );
        assert_eq!(
            tokens[1].token_type,
            TokenType::StringLiteral("world".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_bool_literals() {
        let mut lexer = Lexer::new("true false");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::BoolLiteral(true));
        assert_eq!(tokens[1].token_type, TokenType::BoolLiteral(false));
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn test_nil_literal() {
        let mut lexer = Lexer::new("nil");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Nil);
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_mixed() {
        let mut lexer = Lexer::new("djq x = 10; xn>some_func(x);");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token_type, TokenType::Djq);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Eq);
        assert_eq!(tokens[3].token_type, TokenType::IntLiteral(10));
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::Xnc);
        assert_eq!(tokens[6].token_type, TokenType::Identifier);
        assert_eq!(tokens[7].token_type, TokenType::LParen);
        assert_eq!(tokens[8].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_span_information() {
        let mut lexer = Lexer::new("djq x = 10;\nfnc y() { ret x; }");
        let tokens = lexer.tokenize().unwrap();

        // Check spans for a few tokens as a sample
        assert_eq!(tokens[0].span, Span::new(Position::new(1, 1), Position::new(1, 3))); // djq
        assert_eq!(tokens[1].span, Span::new(Position::new(1, 5), Position::new(1, 5))); // x
        assert_eq!(tokens[3].span, Span::new(Position::new(1, 9), Position::new(1, 10))); // 10
        assert_eq!(tokens[5].span, Span::new(Position::new(2, 1), Position::new(2, 3))); // fnc
    }
}