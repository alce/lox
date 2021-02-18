use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::{Token, TokenKind};

use TokenKind::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ScanError {
    UnexpectedCharacter(char),
    UnterminatedString,
}

pub struct Scanner<'a> {
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
    tokens: Vec<Token<'a>>,
    line: u64,
    col: u64,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            iter: source.char_indices().peekable(),
            tokens: Vec::new(),
            line: 1,
            col: 1,
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn scan(mut self) -> Vec<Token<'a>> {
        while let Some((i, cr)) = self.iter.next() {
            match cr {
                '(' => self.emit(LParen),
                ')' => self.emit(RParen),
                '{' => self.emit(LBrace),
                '}' => self.emit(RBrace),
                ';' => self.emit(Semi),
                ',' => self.emit(Comma),
                '.' => self.emit(Dot),
                '-' => self.emit(Minus),
                '+' => self.emit(Plus),
                '/' => {
                    if self.next_is('/') {
                        while let Some(&(_, n)) = self.iter.peek() {
                            if n == '\n' {
                                break;
                            }
                            self.iter.next();
                        }
                        continue;
                    }
                    self.emit(Slash)
                }
                '*' => self.emit(Star),
                '!' => self.peek_and_emit(Bang, BangEq),
                '=' => self.peek_and_emit(Eq, EqEq),
                '<' => self.peek_and_emit(Lt, LtEq),
                '>' => self.peek_and_emit(Gt, GtEq),
                '"' => self.string(i),
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                ' ' | '\r' => self.col += 1,
                '\t' => self.col += 4, // 4, 8,????
                c if c.is_digit(10) => self.number(i),
                c if c.is_alphabetic() => self.identifier(i),
                _ => self.emit(Error(ScanError::UnexpectedCharacter(cr))),
            }
        }

        self.tokens.push(Token::new(EOF, self.line, self.col));
        self.tokens
    }

    fn string(&mut self, start: usize) {
        while let Some((_, c)) = self.iter.peek() {
            match *c {
                '"' => break,
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                }
                _ => {}
            }
            self.iter.next();
        }

        match self.iter.peek() {
            Some(&(i, _)) => {
                self.emit(Str(&self.source[start + 1..i]));
                self.iter.next();
            }
            None => self.emit(Error(ScanError::UnterminatedString)),
        }
    }

    fn number(&mut self, start: usize) {
        self.consume_digits();

        if self.next_is('.') {
            self.iter.next();
            self.consume_digits();
        }

        let src = &self.source[start..self.end_position()];
        self.emit(Num(src));
    }

    fn identifier(&mut self, start: usize) {
        while let Some((_, c)) = self.iter.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            self.iter.next();
        }

        let src = &self.source[start..self.end_position()];
        self.emit(src.parse().unwrap_or(Ident(src)))
    }

    fn emit(&mut self, kind: TokenKind<'a>) {
        self.col += kind.size();
        self.tokens.push(Token::new(kind, self.line, self.col))
    }

    fn peek_and_emit(&mut self, single: TokenKind<'a>, double: TokenKind<'a>) {
        if self.next_is('=') {
            self.iter.next();
            self.emit(double)
        } else {
            self.emit(single)
        }
    }

    fn consume_digits(&mut self) {
        while let Some((_, c)) = self.iter.peek() {
            if !c.is_digit(10) {
                break;
            }
            self.iter.next();
        }
    }

    fn next_is(&mut self, maybe: char) -> bool {
        if let Some(&(_, c)) = self.iter.peek() {
            if c == maybe {
                return true;
            }
        }
        false
    }

    fn end_position(&mut self) -> usize {
        match self.iter.peek() {
            Some(&(idx, _)) => idx,
            None => self.source.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Vec<TokenKind<'_>> {
        Scanner::new(source).scan().iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_arithmetic() {
        let actual = tokenize("-  (2 + 2) * 8 / 2.2;");

        let expected = [
            Minus,
            LParen,
            Num("2"),
            Plus,
            Num("2"),
            RParen,
            Star,
            Num("8"),
            Slash,
            Num("2.2"),
            Semi,
            EOF,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_strings() {
        let actual = tokenize(
            r#" 
                // don't mind me
                "lox" "hello..."  "nope
            "#,
        );

        let expected = [
            Str("lox"),
            Str("hello..."),
            Error(ScanError::UnterminatedString),
            EOF,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_keywords() {
        let actual = tokenize(
            r#" 
                3 and var foo   fun else 4 //ignore me
                super class or nil while print whale  
            "#,
        );

        let expected = [
            Num("3"),
            And,
            Var,
            Ident("foo"),
            Fun,
            Else,
            Num("4"),
            Super,
            Class,
            Or,
            Nil,
            While,
            Print,
            Ident("whale"),
            EOF,
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_no_tokens() {
        let actual = tokenize(" ");
        assert_eq!(actual, [EOF]);

        let actual = tokenize("// nope");
        assert_eq!(actual, [EOF]);
    }
}
