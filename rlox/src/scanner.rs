use std::fmt;
use std::str::{Chars, FromStr};

use crate::token::{Token, TokenKind};
use TokenKind::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ScanError {
    UnexpectedChar(char),
    UnterminatedString,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::UnexpectedChar(_) => write!(f, "Unexpected character."),
            ScanError::UnterminatedString => write!(f, "Unterminated string."),
        }
    }
}

pub fn tokenize(mut src: &str) -> impl Iterator<Item = Token<'_>> {
    let mut start_line = 1;
    let mut at_end = false;

    std::iter::from_fn(move || {
        if at_end {
            return None;
        }

        if src.is_empty() {
            at_end = true;
            return Some(Token::new(EOF, start_line));
        }

        let (token, consumed) = Scanner::new(src, start_line).scan();
        start_line = token.line;
        src = &src[consumed..];

        Some(token)
    })
    .filter(|t| !t.is_whitespace())
}

struct Scanner<'a> {
    src: &'a str,
    chars: Chars<'a>,
    line: u64,
}

impl<'a> Scanner<'a> {
    fn new(src: &'a str, line: u64) -> Scanner<'a> {
        Scanner {
            src,
            chars: src.chars(),
            line,
        }
    }

    fn scan(&mut self) -> (Token<'a>, usize) {
        let c = self.advance().unwrap();

        let kind = match c {
            c if c.is_ascii_whitespace() => {
                self.advance_while(|c| c.is_ascii_whitespace());
                WHITESPACE
            }
            '/' => match self.peek() {
                Some('/') => {
                    self.advance_while(|c| c != '\n');
                    COMMENT
                }
                _ => SLASH,
            },
            c if is_ident_start(c) => self.identifier(),
            '0'..='9' => self.number(),
            '(' => LEFT_PAREN,
            ')' => RIGHT_PAREN,
            '{' => LEFT_BRACE,
            '}' => RIGHT_BRACE,
            ',' => COMMA,
            ';' => SEMICOLON,
            '.' => DOT,
            '-' => MINUS,
            '+' => PLUS,
            '*' => STAR,
            '!' => self.maybe_double(BANG, BANG_EQUAL),
            '=' => self.maybe_double(EQUAL, EQUAL_EQUAL),
            '>' => self.maybe_double(GREATER, GREATER_EQUAL),
            '<' => self.maybe_double(LESS, LESS_EQUAL),
            '"' => self.string(),
            _ => ERROR(ScanError::UnexpectedChar(c)),
        };

        (Token::new(kind, self.line), self.consumed())
    }

    fn identifier(&mut self) -> TokenKind<'a> {
        self.advance_while(|c| c.is_alphanumeric() || c == '_');
        let src = &self.src[..self.consumed()];
        TokenKind::from_str(src).unwrap_or(IDENTIFIER(src))
    }

    fn number(&mut self) -> TokenKind<'a> {
        self.advance_while(is_number);

        if let (Some('.'), Some(c)) = (self.peek(), self.peek_next()) {
            if is_number(c) {
                self.advance();
                self.advance_while(is_number)
            }
        }

        let src = &self.src[..self.consumed()];
        NUMBER(f64::from_str(src).unwrap())
    }

    fn string(&mut self) -> TokenKind<'a> {
        while let Some(c) = self.advance() {
            if c == '"' {
                return STRING(&self.src[1..self.consumed() - 1]);
            }
        }

        ERROR(ScanError::UnterminatedString)
    }

    fn maybe_double(&mut self, single: TokenKind<'a>, double: TokenKind<'a>) -> TokenKind<'a> {
        if let Some('=') = self.peek() {
            self.advance();
            return double;
        }

        single
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            if c == '\n' {
                self.line += 1;
            };
            c
        })
    }

    fn advance_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while let Some(c) = self.peek() {
            if !f(c) {
                break;
            }
            self.advance();
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.clone().nth(1)
    }

    fn consumed(&self) -> usize {
        self.src.len() - self.chars.as_str().len()
    }
}

fn is_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_number(c: char) -> bool {
    matches!(c, '0'..='9')
}
