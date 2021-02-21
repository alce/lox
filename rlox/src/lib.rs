pub use error::LoxError;

use crate::chunk::Chunk;

mod chunk;
mod compiler;
mod error;
mod parser;
mod scanner;
mod token;
mod vm;

pub fn interpret(source: &str) -> Result<(), LoxError> {
    for token in scanner::tokenize(source).filter(|t| !t.is_whitespace()) {
        println!("{}", token);
    }

    Ok(())
}
