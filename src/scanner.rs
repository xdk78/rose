use result::{Error, Result};
use std::collections::{HashSet, VecDeque};
use std::ops::Index;
use std::str::Chars;
use token::reserved;
use token::Literal;
use token::{Token, TokenType};

pub struct Scanner<'a> {
    src: Chars<'a>,
    peeks: VecDeque<char>,
    lexeme: String,
    line: u64,
    eof: bool,
}

impl<'a> Scanner<'a> {
    /// Creates a new Scanner off a Chars iterator.
    pub fn new(c: Chars<'a>) -> Self {
        Scanner {
            src: c,
            peeks: VecDeque::with_capacity(2),
            lexeme: "".to_string(),
            line: 1,
            eof: false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.eof {
            return None;
        }

        match self.peeks.len() {
            0 => self.src.next(),
            _ => self.peeks.pop_front(),
        }
        .or_else(|| {
            self.eof = true;
            Some('\0')
        })
        .and_then(|c| {
            self.lexeme.push(c);
            Some(c)
        })
    }

    fn peek(&mut self) -> char {
        self.next_char(1)
    }

    fn peek_next(&mut self) -> char {
        self.next_char(2)
    }

    fn next_char(&mut self, n: usize) -> char {
        while self.peeks.len() < n {
            self.src
                .next()
                .or(Some('\0'))
                .map(|c| self.peeks.push_back(c));
        }
        *self.peeks.index(n - 1)
    }

    fn match_advance(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance().unwrap();
            return true;
        }
        false
    }

    fn advance_until(&mut self, c: HashSet<char>) -> char {
        let mut last_char = '\0';

        loop {
            match self.peek() {
                ch if c.contains(&ch) || ch == '\0' => break,
                ch => {
                    last_char = ch;
                    self.advance()
                }
            };
        }
        last_char
    }
}

impl<'a> Scanner<'a> {
    fn static_token(&self, token_type: TokenType) -> Option<Result<Token>> {
        self.literal_token(token_type, None)
    }

    fn match_static_token(&mut self, c: char, m: TokenType, u: TokenType) -> Option<Result<Token>> {
        match self.match_advance(c) {
            true => self.static_token(m),
            false => self.static_token(u),
        }
    }

    fn literal_token(
        &self,
        token_type: TokenType,
        literal: Option<Literal>,
    ) -> Option<Result<Token>> {
        Some(Ok(Token {
            token_type: token_type,
            literal: literal,
            line: self.line,
            lexeme: self.lexeme.clone(),
        }))
    }

    fn number_literal(&mut self) -> Option<Result<Token>> {
        // 10 for decimal
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        if let Ok(literal) = self.lexeme.clone().parse::<f64>() {
            return self.literal_token(TokenType::Number, Some(Literal::Number(literal)));
        }

        self.err("invalid number")
    }

    fn string_literal(&mut self) -> Option<Result<Token>> {
        loop {
            let last = self.advance_until(['\n', '"'].iter().cloned().collect());

            match self.peek() {
                '\n' => self.line += 1,
                '"' if last == '\\' => {
                    self.lexeme.pop();
                }
                '"' => break,
                '\0' => return self.err("unterminated string"),
                _ => return self.err("unexpected character"),
            };

            self.advance();
        }

        self.advance();

        let literal: String = self
            .lexeme
            .clone()
            .chars()
            .skip(1)
            .take(self.lexeme.len() - 2)
            .collect();

        self.literal_token(TokenType::String, Some(Literal::String(literal)))
    }

    fn identifier(&mut self) -> Option<Result<Token>> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let lex: &str = self.lexeme.as_ref();
        let typ = reserved(lex).map_or(TokenType::Identifier, |t| t.clone());

        match typ {
            TokenType::Null => self.literal_token(typ, Some(Literal::Null)),
            TokenType::True => self.literal_token(typ, Some(Literal::Boolean(true))),
            TokenType::False => self.literal_token(typ, Some(Literal::Boolean(false))),
            _ => self.static_token(typ),
        }
    }

    fn line_comment(&mut self) {
        self.advance_until(['\n'].iter().cloned().collect());
        self.lexeme.clear();
    }

    fn block_comment(&mut self) {
        // Move to  `*`
        self.advance();

        loop {
            let last = self.advance_until(['\n', '/'].iter().cloned().collect());
            let next = self.peek();
            match (last, next) {
                (_, '\n') => self.line += 1,
                ('*', '/') => {
                    // Move to another `*`
                    self.advance();
                    // Move to last `/`
                    self.advance();
                    break;
                }
                (_, '\0') => break,
                (_, _) => (),
            }
            self.advance();
        }

        self.lexeme.clear();
    }

    fn err(&self, msg: &str) -> Option<Result<Token>> {
        Some(Err(Error::Lexical(
            self.line,
            msg.to_string(),
            self.lexeme.clone(),
        )
        .boxed()))
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        use token::TokenType::*;

        if self.eof {
            return None;
        }

        self.lexeme.clear();

        let c = self.advance().unwrap();
        match c {
            '(' => self.static_token(LeftParen),
            ')' => self.static_token(RightParen),
            '{' => self.static_token(LeftBrace),
            '}' => self.static_token(RightBrace),
            ',' => self.static_token(Comma),
            '.' => self.static_token(Dot),
            ';' => self.static_token(Semicolon),
            '\0' => self.static_token(EOF),

            '-' => self.match_static_token('=', MinusEqual, Minus),
            '+' => self.match_static_token('=', PlusEqual, Plus),
            '!' => self.match_static_token('=', BangEqual, Bang),
            '=' => self.match_static_token('=', EqualEqual, Equal),
            '<' => self.match_static_token('=', LessEqual, Less),
            '>' => self.match_static_token('=', GreaterEqual, Greater),
            '*' => self.match_static_token('=', StarEqual, Star),

            '"' => self.string_literal(),

            '/' => {
                if self.match_advance('/') {
                    self.line_comment();
                    self.next()
                } else if self.match_advance('*') {
                    self.block_comment();
                    self.next()
                } else {
                    self.match_static_token('=', SlashEqual, Slash)
                }
            }

            c if c.is_whitespace() => {
                self.lexeme.clear();
                if c == '\n' {
                    self.line += 1;
                }
                self.next()
            }

            c if c.is_digit(10) => self.number_literal(),
            c if is_alphanumeric(c) => self.identifier(),

            _ => self.err("unexpected character"),
        }
    }
}

pub trait TokenIterator<'a> {
    fn tokens(self) -> Scanner<'a>;
}

impl<'a> TokenIterator<'a> for Chars<'a> {
    fn tokens(self) -> Scanner<'a> {
        Scanner::new(self)
    }
}

/// Checks if char is alphanumeric
fn is_alphanumeric(c: char) -> bool {
    return c.is_digit(36) || c == '_';
}
