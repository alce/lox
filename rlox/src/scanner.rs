use crate::token::{Token, TokenKind};
use std::str::{Chars, FromStr};
use TokenKind::*;

#[derive(Debug, PartialEq)]
pub enum ScanError {
    UnexpectedChar,
    UnterminatedString,
}

pub fn tokenize(mut src: &str) -> impl Iterator<Item = Token<'_>> {
    let mut start_line = 1;

    std::iter::from_fn(move || {
        if src.is_empty() {
            return None;
        }
        let (token, consumed) = Scanner::new(src, start_line).scan();
        start_line = token.line;
        src = &src[consumed..];

        Some(token)
    })
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
            _ => ERROR(ScanError::UnexpectedChar),
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
                return STRING(&self.src[..self.consumed()]);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(src: &str) -> Vec<TokenKind<'_>> {
        super::tokenize(src)
            .filter(|t| !t.is_whitespace())
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn test_arithmetic() {
        let actual = tokenize("-  (2 + 2) * 8 / 2.2;");

        let expected = [
            MINUS,
            LEFT_PAREN,
            NUMBER(2.0),
            PLUS,
            NUMBER(2.0),
            RIGHT_PAREN,
            STAR,
            NUMBER(8.0),
            SLASH,
            NUMBER(2.2),
            SEMICOLON,
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
            STRING("\"lox\""),
            STRING("\"hello...\""),
            ERROR(ScanError::UnterminatedString),
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
            NUMBER(3.0),
            AND,
            VAR,
            IDENTIFIER("foo"),
            FUN,
            ELSE,
            NUMBER(4.0),
            SUPER,
            CLASS,
            OR,
            NIL,
            WHILE,
            PRINT,
            IDENTIFIER("whale"),
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_no_tokens() {
        let actual = tokenize(" ");
        assert_eq!(actual, []);

        let actual = tokenize("// nope");
        assert_eq!(actual, []);
    }

    #[test]
    fn line_numbers() {
        let src = r#"
            foo
            22.33
            bar
        "#;

        let actual = super::tokenize(src)
            .filter(|t| !t.is_whitespace())
            .map(|t| t.line)
            .collect::<Vec<_>>();

        assert_eq!(actual, [2, 3, 4]);
    }
}
