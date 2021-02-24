mod ast;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;
mod visitor;

use crate::interpreter::Interpreter;
pub use error::LoxError;

pub fn interpret(source: &str) -> Result<(), LoxError> {
    let expr = parser::parse(source);
    let mut interpreter = Interpreter {};
    let val = interpreter.interpret(&expr);

    println!("{}", val);

    Ok(())
}
