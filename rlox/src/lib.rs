mod ast;
mod error;
mod parser;
mod scanner;
mod token;
mod visitor;

pub use error::LoxError;

pub fn interpret(source: &str) -> Result<(), LoxError> {
    println!("{}", parser::parse(source));

    Ok(())
}
