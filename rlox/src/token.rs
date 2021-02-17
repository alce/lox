use crate::scanner::ScanError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: u64,
    pub col: u64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind<'a> {
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Ident(&'a str),
    Str(&'a str),
    Number(&'a str),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error(ScanError),
    EOF,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, line: u64, col: u64) -> Self {
        Token { kind, line, col }
    }
}

impl<'a> FromStr for TokenKind<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TokenKind::*;
        match s {
            "and" => Ok(And),
            "class" => Ok(Class),
            "else" => Ok(Else),
            "false" => Ok(False),
            "for" => Ok(For),
            "fun" => Ok(Fun),
            "if" => Ok(If),
            "nil" => Ok(Nil),
            "or" => Ok(Or),
            "print" => Ok(Print),
            "return" => Ok(Return),
            "super" => Ok(Super),
            "this" => Ok(This),
            "true" => Ok(True),
            "var" => Ok(Var),
            "while" => Ok(While),
            _ => Err(()),
        }
    }
}

impl<'a> TokenKind<'a> {
    pub fn size(&self) -> u64 {
        use self::TokenKind::*;

        match self {
            LeftParen | RightParen | LeftBrace | RightBrace | Comma | Dot | Minus | Plus
            | Semicolon | Slash | Star | Bang | Equal | Greater | Less => 1,
            BangEqual | EqualEqual | GreaterEqual | LessEqual => 2,
            Ident(s) => s.len() as u64,
            Str(s) => s.len() as u64,
            Number(s) => s.len() as u64,
            If | Or => 2,
            And | Fun | For | Var | Nil => 3,
            This | Else | True => 4,
            Super | While | Print | False | Class => 5,
            Return => 6,
            EOF | Error(_) => 0,
        }
    }
}
