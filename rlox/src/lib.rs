mod ast;
mod clock;
mod env;
mod error;
mod function;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;
mod visitor;

use crate::ast::Stmt;
pub use crate::interpreter::Interpreter;
pub use error::LoxError;

pub fn interpret(source: &str) -> Result<(), LoxError> {
    let stmts = parse(source)?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(stmts).map_err(Into::into)
}

pub fn parse(source: &str) -> Result<Vec<Stmt>, LoxError> {
    let (stmts, errors) = parser::parse(source);

    if errors.is_empty() {
        return Ok(stmts);
    }

    for error in &errors[..errors.len() - 1] {
        eprintln!("{}", error);
    }

    Err(errors.last().cloned().unwrap())
}
