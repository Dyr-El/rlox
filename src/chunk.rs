use std::usize;

use crate::values::Value;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Byte(u8);

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum OpCode {
    OpConstant,
    OpConstantLong,
    OpReturn,
}

impl From<OpCode> for Byte {
    fn from(m: OpCode) -> Byte {
        Byte(m as u8)
    }
}

impl From<usize> for Byte {
    fn from(m: usize) -> Byte {
        Byte(m as u8)
    }
}

impl From<Byte> for usize {
    fn from(byte: Byte) -> usize {
        byte.0 as usize
    }
}

impl TryFrom<Byte> for OpCode {
    type Error = ();

    fn try_from(byte: Byte) -> Result<Self, Self::Error> {
        const OP_CONSTANT_BYTE: Byte = Byte(OpCode::OpConstant as u8);
        const OP_CONSTANT_LONG_BYTE: Byte = Byte(OpCode::OpConstantLong as u8);
        const OP_RETURN_BYTE: Byte = Byte(OpCode::OpReturn as u8);
        match byte {
            OP_CONSTANT_BYTE => Ok(OpCode::OpConstant),
            OP_CONSTANT_LONG_BYTE => Ok(OpCode::OpConstantLong),
            OP_RETURN_BYTE => Ok(OpCode::OpReturn),
            _ => Err(()),
        }
    }
}

pub struct Chunk {
    code: Vec<Byte>,
    values: Vec<Value>,
    lines: Vec<(usize, usize)>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            values: vec![],
            lines: vec![],
        }
    }
    pub fn write_code(&mut self, byte: Byte, line: usize) {
        self.code.push(byte);
        if let Some(last_line) = self.lines.last() {
            if last_line.0 == line {
                self.lines.pop();
            }
            self.lines.push((line, self.code.len()))
        } else {
            self.lines.push((line, self.code.len()))
        }
    }
    pub fn write_const(&mut self, value: Value, line: usize) {
        let idx = self.add_constant(value);
        if idx > 255 {
            self.write_code(Byte::from(OpCode::OpConstantLong), line);
            self.write_code(Byte::from((idx >> 16) & 0xFF), line);
            self.write_code(Byte::from((idx >> 8) & 0xFF), line);
            self.write_code(Byte::from(idx & 0xFF), line);
        } else {
            self.write_code(Byte::from(OpCode::OpConstant), line);
            self.write_code(Byte::from(idx & 0xFF), line);
        }
    }
    pub fn code_size(&self) -> usize {
        self.code.len()
    }
    pub fn read_code(&self, idx: usize) -> Byte {
        self.code[idx]
    }
    pub fn read_line(&self, idx: usize) -> usize {
        for (line, code_idx) in &self.lines {
            if idx < *code_idx {
                return *line;
            }
        }
        0
    }
    fn add_constant(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
    pub fn read_constant(&self, idx: usize) -> Value {
        self.values[idx]
    }
}
