mod chunk;
mod error;
mod parser;
mod scanner;
mod token;
mod vm;

use crate::chunk::{Chunk, OpCode};
use crate::scanner::Scanner;

pub fn interpret(source: &str) {
    let tokens = Scanner::new(source.trim()).scan();
    println!("Tokens: {:?}", tokens);
}

#[allow(unused)]
pub fn chunk() {
    let mut c = Chunk::new();
    c.write_constant(1.2, 123);
    c.write(OpCode::Return, 123);
    c.disassemble("test chunk");
}