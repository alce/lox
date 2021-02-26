mod ast;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;
mod visitor;

use crate::ast::Stmt;
pub use crate::interpreter::Interpreter;
use crate::parser::ParseError;
pub use error::LoxError;

pub fn interpret(source: &str) -> Result<(), LoxError> {
    let stmts = parser::parse(source)?;
    let mut interpreter = Interpreter::new();

    interpreter.interpret(stmts).map_err(Into::into)
}

pub fn parse(source: &str) -> Result<Vec<Stmt>, ParseError> {
    parser::parse(source)
}
