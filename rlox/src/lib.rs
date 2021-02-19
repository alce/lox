#![allow(unused)]
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
    let tokens = scanner::tokenize(source);
    let mut chunk = Chunk::new();
    compiler::compile(&tokens, &mut chunk)?;
    vm::interpret(chunk)
}
