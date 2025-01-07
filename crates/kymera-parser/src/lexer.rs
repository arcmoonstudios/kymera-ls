use crate::err::{KymeraParserError, Result};
use crate::position::{Position, Span};

/// Represents the types of tokens in the Kymera language.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Des,    // Structure definition
    Enum,   // Enumeration definition
    Imp,    // Implementation block
    Fnc,    // Function definition
    Forma,  // Struct declaration
    Ret,    // Return statement
    Wyo,    // While loop
    Ate,    // If statement
    As,     // Import alias
    Idit,   // Variable declaration
    Spacs,  // Scope resolution operator (::)
    Soy,    // Self-reference operator
    Snc,    // Synchronous operator
    Xnc,    // Asynchronous operator
    Spro,   // Async/await operator
    Res,    // Result type
    Djq,    // Variable declaration
    Rev,    // Error propagation/handling
    Mth,    // Match statement
    Spa,    // For/foreach loop
    Optn,   // Option type
    Stilo,  // Immutable string slice
    Strng,  // Mutable string
    Muta,   // Mutable designator
    Nmut,   // Immutable designator
    Ifz,    // Interface definition
    I8,     // 8-bit signed integer
    I16,    // 16-bit signed integer
    I32,    // 32-bit signed integer
    I64,    // 64-bit signed integer
    I128,   // 128-bit signed integer
    Isz,    // Architecture-dependent signed integer
    U8,     // 8-bit unsigned integer
    U16,    // 16-bit unsigned integer
    U32,    // 32-bit unsigned integer
    U64,    // 64-bit unsigned integer
    U128,   // 128-bit unsigned integer
    Usz,    // Architecture-dependent unsigned integer
    F32,    // 32-bit floating point
    F64,    // 64-bit floating point
    Prnt,   // Print statement
    Cmt,    // Line comment
    Bmt,    // Block comment
    Dmt,    // Documentation comment
    Verx,   // Verbose built-in AI debugger

    // Identifiers
    Identifier(String),

    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Nil,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    Eq,
    EqEq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Not,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Pydes,  // Python import
    Rudes,  // Rust import

    // Special
    Eof,
}

/// Represents a token with its type, value, and position in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of the token.
    pub token_type: TokenType,
    /// The lexeme (text) of the token.
    pub lexeme: String,
    /// The location of the token in the source code.
    pub span: Span,
}

/// Lexer for the Kymera language.
pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    current_pos: Position,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            current_pos: Position::new(1, 1, 0),
        }
    }

    /// Returns the source code being lexed.
    pub fn source(&self) -> &str {
        self.source
    }

    /// Returns the current position in the source code.
    pub fn position(&self) -> Position {
        self.current_pos
    }

    /// Tokenizes the entire source code.
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token.clone());
            if token.token_type == TokenType::Eof {
                break;
            }
        }
        Ok(tokens)
    }

    /// Returns the next token from the source code.
    pub fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();
        
        let start_pos = self.current_pos;
        let next_char = self.peek();

        match next_char {
            None => Ok(Some(self.make_token(TokenType::Eof, String::new(), start_pos))),
            Some(c) => {
                match c {
                    '0'..='9' => self.scan_number(),
                    'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),
                    '"' => self.scan_string(),
                    '/' => {
                        self.advance();
                        match self.peek() {
                            Some('/') => {
                                self.skip_line_comment();
                                self.next_token()
                            }
                            Some('*') => {
                                self.skip_block_comment()?;
                                self.next_token()
                            }
                            _ => Ok(Some(self.make_token(TokenType::Slash, "/".to_string(), start_pos)))
                        }
                    }
                    '=' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Ok(Some(self.make_token(TokenType::EqEq, "==".to_string(), start_pos)))
                        } else {
                            Ok(Some(self.make_token(TokenType::Eq, "=".to_string(), start_pos)))
                        }
                    }
                    '+' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Ok(Some(self.make_token(TokenType::PlusEq, "+=".to_string(), start_pos)))
                        } else {
                            Ok(Some(self.make_token(TokenType::Plus, "+".to_string(), start_pos)))
                        }
                    }
                    '-' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Ok(Some(self.make_token(TokenType::MinusEq, "-=".to_string(), start_pos)))
                        } else {
                            Ok(Some(self.make_token(TokenType::Minus, "-".to_string(), start_pos)))
                        }
                    }
                    '*' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Ok(Some(self.make_token(TokenType::StarEq, "*=".to_string(), start_pos)))
                        } else {
                            Ok(Some(self.make_token(TokenType::Star, "*".to_string(), start_pos)))
                        }
                    }
                    '(' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::LParen, "(".to_string(), start_pos)))
                    }
                    ')' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::RParen, ")".to_string(), start_pos)))
                    }
                    '{' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::LBrace, "{".to_string(), start_pos)))
                    }
                    '}' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::RBrace, "}".to_string(), start_pos)))
                    }
                    '[' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::LBracket, "[".to_string(), start_pos)))
                    }
                    ']' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::RBracket, "]".to_string(), start_pos)))
                    }
                    ',' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::Comma, ",".to_string(), start_pos)))
                    }
                    '.' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::Dot, ".".to_string(), start_pos)))
                    }
                    ';' => {
                        self.advance();
                        Ok(Some(self.make_token(TokenType::Semicolon, ";".to_string(), start_pos)))
                    }
                    ':' => {
                        self.advance();
                        if self.peek() == Some('>') {
                            self.advance();
                            Ok(Some(self.make_token(TokenType::Spacs, ":>".to_string(), start_pos)))
                        } else {
                            Ok(Some(self.make_token(TokenType::Colon, ":".to_string(), start_pos)))
                        }
                    }
                    _ => Err(self.error(format!("Unexpected character: {}", c)))
                }
            }
        }
    }

    /// Scans a string literal.
    fn scan_string(&mut self) -> Result<Option<Token>> {
        let start_pos = self.current_pos;
        let mut string = String::new();
        
        self.advance(); // Skip opening quote
        
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // Skip closing quote
                return Ok(Some(self.make_token(
                    TokenType::StringLiteral(string.clone()),
                    format!("\"{}\"", string),
                    start_pos
                )));
            }
            
            if c == '\\' {
                self.advance();
                match self.peek() {
                    Some('n') => { string.push('\n'); self.advance(); }
                    Some('r') => { string.push('\r'); self.advance(); }
                    Some('t') => { string.push('\t'); self.advance(); }
                    Some('\\') => { string.push('\\'); self.advance(); }
                    Some('"') => { string.push('"'); self.advance(); }
                    Some(c) => return Err(self.error(format!("Invalid escape sequence: \\{}", c))),
                    None => return Err(self.error("Unterminated escape sequence")),
                }
            } else {
                string.push(self.advance().unwrap());
            }
        }
        
        Err(self.error("Unterminated string literal"))
    }

    /// Skips a line comment.
    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Skips a block comment.
    fn skip_block_comment(&mut self) -> Result<()> {
        self.advance(); // Skip *
        let mut nesting = 1;
        
        while nesting > 0 {
            match self.peek() {
                Some('/') => {
                    self.advance();
                    if self.peek() == Some('*') {
                        self.advance();
                        nesting += 1;
                    }
                }
                Some('*') => {
                    self.advance();
                    if self.peek() == Some('/') {
                        self.advance();
                        nesting -= 1;
                    }
                }
                Some(_) => {
                    self.advance();
                }
                None => return Err(self.error("Unterminated block comment")),
            }
        }
        
        Ok(())
    }

    /// Returns the next character without consuming it.
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Advances to the next character and returns it.
    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(c) = c {
            if c == '\n' {
                self.current_pos.newline();
            } else {
                self.current_pos.advance();
            }
        }
        c
    }

    /// Creates a token with the given type and lexeme.
    fn make_token(&self, token_type: TokenType, lexeme: String, start_pos: Position) -> Token {
        Token {
            token_type,
            lexeme,
            span: Span::new(start_pos, self.current_pos),
        }
    }

    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    /// Scans an identifier or keyword.
    fn scan_identifier(&mut self) -> Result<Option<Token>> {
        let start_pos = self.current_pos;
        let mut lexeme = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                lexeme.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let token_type = match lexeme.as_str() {
            "pydes" => TokenType::Pydes,
            "rudes" => TokenType::Rudes,
            "des" => TokenType::Des,
            "enum" => TokenType::Enum,
            "imp" => TokenType::Imp,
            "fnc" => TokenType::Fnc,
            "forma" => TokenType::Forma,
            "ret" => TokenType::Ret,
            "wyo" => TokenType::Wyo,
            "ate" => TokenType::Ate,
            "as" => TokenType::As,
            "idit" => TokenType::Idit,
            "djq" => TokenType::Djq,
            "rev" => TokenType::Rev,
            "mth" => TokenType::Mth,
            "spa" => TokenType::Spa,
            "optn" => TokenType::Optn,
            "stilo" => TokenType::Stilo,
            "strng" => TokenType::Strng,
            "muta" => TokenType::Muta,
            "nmut" => TokenType::Nmut,
            "ifz" => TokenType::Ifz,
            "i8" => TokenType::I8,
            "i16" => TokenType::I16,
            "i32" => TokenType::I32,
            "i64" => TokenType::I64,
            "i128" => TokenType::I128,
            "isz" => TokenType::Isz,
            "u8" => TokenType::U8,
            "u16" => TokenType::U16,
            "u32" => TokenType::U32,
            "u64" => TokenType::U64,
            "u128" => TokenType::U128,
            "usz" => TokenType::Usz,
            "f32" => TokenType::F32,
            "f64" => TokenType::F64,
            "prnt" => TokenType::Prnt,
            "true" => TokenType::BoolLiteral(true),
            "false" => TokenType::BoolLiteral(false),
            "nil" => TokenType::Nil,
            _ => TokenType::Identifier(lexeme.clone()),
        };

        Ok(Some(self.make_token(token_type, lexeme, start_pos)))
    }

    /// Scans a number literal.
    fn scan_number(&mut self) -> Result<Option<Token>> {
        let start_pos = self.current_pos;
        let mut lexeme = String::new();
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                lexeme.push(self.advance().unwrap());
            } else if c == '.' && !is_float {
                is_float = true;
                lexeme.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let token_type = if is_float {
            TokenType::FloatLiteral(lexeme.parse().map_err(|_| self.error("Invalid float literal"))?)
        } else {
            TokenType::IntLiteral(lexeme.parse().map_err(|_| self.error("Invalid integer literal"))?)
        };

        Ok(Some(self.make_token(token_type, lexeme, start_pos)))
    }

    /// Creates an error with the given message at the current position.
    fn error(&self, message: impl Into<String>) -> KymeraParserError {
        KymeraParserError::Lexer {
            message: message.into(),
            span: Span::new(self.current_pos, self.current_pos),
        }
    }
}