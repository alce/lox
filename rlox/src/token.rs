use std::fmt;
use std::str::FromStr;

use crate::scanner::ScanError;

#[derive(PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: u64,
}

#[allow(bad_style)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind<'a> {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(&'a str),
    STRING(&'a str),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    WHITESPACE,
    COMMENT,

    ERROR(ScanError),

    EOF,
}

impl<'a> TokenKind<'a> {
    #[allow(unused)]
    pub fn is_operator(&self) -> bool {
        use TokenKind::*;
        matches!(self, MINUS | PLUS | SLASH | STAR | LEFT_PAREN)
    }

    pub fn prefix_bp(&self) -> ((), u8) {
        match self {
            TokenKind::PLUS | TokenKind::MINUS => ((), 15),
            TokenKind::BANG => ((), 16),
            _ => panic!("no prefix bp for {} yet", self),
        }
    }

    pub fn infix_bp(&self) -> Option<(u8, u8)> {
        use TokenKind::*;
        match self {
            EQUAL_EQUAL | BANG_EQUAL => Some((1, 2)),
            GREATER | GREATER_EQUAL | LESS | LESS_EQUAL => Some((3, 4)),
            MINUS | PLUS => Some((5, 6)),
            STAR | SLASH => Some((7, 8)),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        use TokenKind::*;
        matches!(self, NUMBER(_) | NIL | STRING(_) | TRUE | FALSE)
    }
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, line: u64) -> Token<'_> {
        Token { kind, line }
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self.kind, TokenKind::COMMENT | TokenKind::WHITESPACE)
    }
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenKind::STRING(s) => write!(f, "STRING {:?} {}", s, s),
            TokenKind::NUMBER(s) => write!(f, "NUMBER {} {:?}", s, s),
            TokenKind::IDENTIFIER(s) => write!(f, "IDENTIFIER {} null", s),
            _ => write!(f, "{:?} {} null", self.kind, self.kind),
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenKind::STRING(s) => write!(f, "{}", s),
            TokenKind::NUMBER(s) => write!(f, "{}", s,),
            TokenKind::IDENTIFIER(s) => write!(f, "{}", s),
            _ => write!(f, "{}", self.kind),
        }
    }
}

impl<'a> FromStr for TokenKind<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TokenKind::*;

        match s {
            "and" => Ok(AND),
            "class" => Ok(CLASS),
            "else" => Ok(ELSE),
            "false" => Ok(FALSE),
            "for" => Ok(FOR),
            "fun" => Ok(FUN),
            "if" => Ok(IF),
            "nil" => Ok(NIL),
            "or" => Ok(OR),
            "print" => Ok(PRINT),
            "return" => Ok(RETURN),
            "super" => Ok(SUPER),
            "this" => Ok(THIS),
            "true" => Ok(TRUE),
            "var" => Ok(VAR),
            "while" => Ok(WHILE),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;

        if let NUMBER(n) = self {
            return write!(f, " {}", n);
        }

        f.write_str(match self {
            LEFT_PAREN => "(",
            RIGHT_PAREN => ")",
            LEFT_BRACE => "{",
            RIGHT_BRACE => "}",
            SEMICOLON => ";",
            COMMA => ",",
            DOT => ".",
            MINUS => "-",
            PLUS => "+",
            SLASH => "/",
            STAR => "*",
            BANG => "!=",
            BANG_EQUAL => "!=",
            EQUAL => "=",
            EQUAL_EQUAL => "==",
            GREATER => ">",
            GREATER_EQUAL => ">=",
            LESS => "<",
            LESS_EQUAL => "<=",
            AND => "and",
            CLASS => "class",
            ELSE => "else",
            FALSE => "false",
            FUN => "fun",
            FOR => "for",
            IF => "if",
            NIL => "nil",
            OR => "or",
            PRINT => "print",
            RETURN => "return",
            SUPER => "super",
            THIS => "this",
            TRUE => "true",
            VAR => "var",
            WHILE => "while",
            IDENTIFIER(s) => s,
            STRING(s) => s,
            EOF => "eof",
            ERROR(_) => "err",
            _ => "%",
        })
    }
}
