use std::collections::HashMap;
use std::default;
use std::fmt;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u64,
}

impl default::Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // Literals
    Identifier,
    String,
    Number,
    Boolean,

    // Keywords
    Else,
    True,
    False,
    Fun,
    For,
    If,
    Null,
    Print,
    Mut,
    Const,
    While,
    EOF,
}

#[derive(PartialEq)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

/// Returns a matching Token Type if a keyword is reserved
pub fn reserved(keyword: &str) -> Option<&TokenType> {
    RESERVED.get(keyword)
}

lazy_static! {
    static ref RESERVED: HashMap<&'static str, TokenType> = [
        ("else", TokenType::Else),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("null", TokenType::Null),
        ("print", TokenType::Print),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("mut", TokenType::Mut),
        ("const", TokenType::Const),
        ("while", TokenType::While),
        ("null", TokenType::Null),
    ]
    .iter()
    .cloned()
    .collect();
}
