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
    c.write_constant(3.4, 123);
    c.write(OpCode::Add, 123);
    c.write_constant(5.6, 123);

    c.write(OpCode::Divide, 123);
    c.write(OpCode::Negate, 123);

    c.write(OpCode::Return, 123);

    let mut vm = vm::Vm::new(c);
    vm.interpret().unwrap();

    // c.disassemble("test chunk");
}
