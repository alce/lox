use crate::token::{Token, TokenKind};
use crate::{Chunk, LoxError};

pub enum ParseError {
    EndOfStream,
    UnexpectedToken,
}

pub(crate) struct Parser<'a> {
    tokens: &'a [Token<'a>],
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser { tokens }
    }

    pub fn peek(&mut self) -> Option<&'a Token<'a>> {
        self.tokens.get(0)
    }
}
