#![allow(unused)]

use crate::chunk::Chunk;
use crate::token::Token;

pub enum ParseError {
    EndOfStream,
    UnexpectedToken,
}

pub fn parse<'a>(tokens: &'a [Token<'a>], chunk: &mut Chunk) {
    todo!()
}

struct Parser<'a> {
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
