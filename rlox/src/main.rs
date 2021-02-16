enum OpCode {
    Return,
    Unknown,
}

impl From<u8> for OpCode {
    fn from(n: u8) -> Self {
        match n {
            0 => OpCode::Return,
            _ => OpCode::Unknown,
        }
    }
}

#[derive(Debug)]
struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    fn write(&mut self, op: OpCode) {
        self.code.push(op as u8);
    }

    fn len(&self) -> usize {
        self.code.len()
    }
}

impl Chunk {
    fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        let op: OpCode = self.code[offset].into();

        match op {
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
            OpCode::Unknown => {
                println!("Unknown opcode {}", self.code[offset]);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);
    chunk.disassemble("test chunk");
}
