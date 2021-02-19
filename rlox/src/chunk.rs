use std::fmt;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
    Negate,
}

pub type Value = f64;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn get_constant(&self, idx: usize) -> Value {
        self.constants[idx]
    }

    pub fn get_code(&self, idx: usize) -> u8 {
        self.code[idx]
    }

    pub fn write<T: Into<u8>>(&mut self, data: T, line: usize) {
        self.code.push(data.into());
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) -> usize {
        self.constants.push(value);
        let idx = self.constants.len() - 1;
        self.write(OpCode::Constant, line);
        self.write(idx as u8, line);
        idx
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, mut offset: usize) -> usize {
        use OpCode::*;

        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let op: OpCode = self.code[offset].into();

        match op {
            Return | Negate | Add | Subtract | Multiply | Divide => {
                println!("{}", op);
                offset += 1;
            }
            Constant => {
                let constant = self.code[offset + 1];
                println!(
                    "{:-16} {:4} '{}'",
                    op, constant, self.constants[constant as usize]
                );
                offset += 2;
            }
        }
        offset
    }
}

impl OpCode {
    pub fn is_arithmetic(&self) -> bool {
        use OpCode::*;
        matches!(self, Add | Divide | Multiply | Subtract)
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use OpCode::*;

        let name = match self {
            Constant => "OP_CONSTANT",
            Add => "OP_ADD",
            Subtract => "OP_SUBTRACT",
            Multiply => "OP_MULTIPLY",
            Divide => "OP_DIVIDE",
            Return => "OP_RETURN",
            Negate => "OP_NEGATE",
        };

        f.write_str(name)
    }
}

impl From<u8> for OpCode {
    fn from(n: u8) -> Self {
        match n {
            0 => OpCode::Constant,
            1 => OpCode::Add,
            2 => OpCode::Subtract,
            3 => OpCode::Multiply,
            4 => OpCode::Divide,
            5 => OpCode::Return,
            6 => OpCode::Negate,
            _ => panic!("Unknown opcode {}", n),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        match op {
            OpCode::Constant => 0,
            OpCode::Add => 1,
            OpCode::Subtract => 2,
            OpCode::Multiply => 3,
            OpCode::Divide => 4,
            OpCode::Return => 5,
            OpCode::Negate => 6,
        }
    }
}
