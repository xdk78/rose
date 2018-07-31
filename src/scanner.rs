use result::{Error, Result};
use std::collections::{HashSet, VecDeque};
use std::ops::Index;
use std::str::Chars;
use token::Literal;
use token::{Token, TokenType};

pub struct Scanner<'a> {
    src: Chars<'a>,
    peeks: VecDeque<char>,
    lexeme: String,
    line: u64,
    eof: bool,
}

pub fn new(c: Chars) -> Scanner {
    Scanner {
        src: c,
        peeks: VecDeque::with_capacity(2),
        lexeme: "".to_string(),
        line: 1,
        eof: false,
    }
}

impl<'a> Scanner<'a> {
    fn advance(&mut self) -> Option<char> {
        if self.eof {
            return None;
        }

        match self.peeks.len() {
            0 => self.src.next(),
            _ => self.peeks.pop_front(),
        }.or_else(|| {
            self.eof = true;
            Some('\0')
        })
            .and_then(|c| {
                self.lexeme.push(c);
                Some(c)
            })
    }

    fn peek(&mut self) -> char {
        self.peek_next(1)
    }

    fn peek_next(&mut self, n: usize) -> char {
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

    fn err(&self, msg: &str) -> Option<Result<Token>> {
        Some(Err(Error::Lexical(
            self.line,
            msg.to_string(),
            self.lexeme.clone(),
        ).boxed()))
    }

    fn match_static_token(&mut self, c: char, m: TokenType, u: TokenType) -> Option<Result<Token>> {
        match self.match_advance(c) {
            true => self.static_token(m),
            false => self.static_token(u),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }
        use token::TokenType::*;
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

            '/' => {
                if self.match_advance('/') {
                    self.advance_until(['\n'].iter().cloned().collect());
                    return self.next();
                }
                self.match_static_token('=', SlashEqual, Slash)
            }

            _ => self.err("unexpected character"),
        }
    }
}

pub trait TokenIterator<'a> {
    fn tokens(self) -> Scanner<'a>;
}

impl<'a> TokenIterator<'a> for Chars<'a> {
    fn tokens(self) -> Scanner<'a> {
        new(self)
    }
}
