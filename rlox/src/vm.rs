#![allow(unused)]

use crate::chunk::{Chunk, OpCode, Value};
use crate::error::Result;

const STACK_MAX: usize = 256;

use OpCode::*;

#[derive(Debug)]
pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self) -> Result {
        self.run()
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn run(&mut self) -> Result {
        loop {
            self.trace();

            match self.read_opcode() {
                Return => {
                    println!("{:?}", self.pop());
                    return Ok(());
                }
                Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Negate => {
                    let val = self.pop();
                    self.push(-val);
                }
                op if op.is_arithmetic() => self.apply(op),
                _ => unreachable!("unknown opcode"),
            }
        }
    }

    fn apply(&mut self, op: OpCode) {
        let (b, a) = self.pop_two();

        let res = match op {
            Add => a + b,
            Subtract => a - b,
            Divide => a / b,
            Multiply => a * b,
            _ => unreachable!("not an arithmetic opcode"),
        };

        self.push(res);
    }

    fn pop_two(&mut self) -> (Value, Value) {
        (self.pop(), self.pop())
    }

    fn read_constant(&mut self) -> Value {
        let b = self.read_byte();
        self.chunk.get_constant(b as usize)
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.get_code(self.ip - 1)
    }

    fn read_opcode(&mut self) -> OpCode {
        self.read_byte().into()
    }

    fn trace(&mut self) {
        print!("          ");
        self.stack[..self.stack_top]
            .iter()
            .for_each(|v| print!("[ {:?} ]", v));
        println!();
        self.chunk.disassemble_instruction(self.ip);
    }
}
