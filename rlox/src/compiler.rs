#![allow(unused)]
use crate::chunk::{OpCode, Value};
use crate::parser::Parser;
use crate::token::{Token, TokenKind};
use crate::{Chunk, LoxError};
use std::str::FromStr;

use rules::Precedence;

fn grouping(p: &mut Parser) {
    todo!()
}

fn unary(p: &mut Parser) {
    todo!()
}

fn number(p: &mut Parser) {
    todo!()
}

fn parse_precedence(p: &mut Parser, precedence: Precedence) {
    todo!()
}

fn expression(p: &mut Parser) {
    parse_precedence(p, Precedence::Assignment)
}

pub fn compile<'a>(tokens: &'a [Token<'a>], chunk: &mut Chunk) -> Result<(), LoxError> {
    let mut parser = Parser::new(tokens);
    expression(&mut parser);
    Ok(())
}

mod rules {
    use super::grouping;
    use crate::parser::Parser;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
    pub enum Precedence {
        None,
        Assignment,
        Or,
        And,
        Equality,
        Comparison,
        Term,
        Factor,
        Unary,
        Call,
        Primary,
    }

    pub(super) type ParseFn = fn(&mut Parser);

    pub(super) struct ParseRule {
        pub prefix: Option<ParseFn>,
        pub infix: Option<ParseFn>,
        pub precedence: Precedence,
    }
}
