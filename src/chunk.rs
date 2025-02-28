use std::usize;

use crate::values::Value;

#[derive(PartialEq, Clone, Copy)]
pub struct Byte(u8);

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum OpCode {
    OpConstant,
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
        const OP_RETURN_BYTE: Byte = Byte(OpCode::OpReturn as u8);
        match byte {
            OP_CONSTANT_BYTE => Ok(OpCode::OpConstant),
            OP_RETURN_BYTE => Ok(OpCode::OpReturn),
            _ => Err(()),
        }
    }
}

pub struct Chunk {
    code: Vec<Byte>,
    values: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            values: vec![],
        }
    }
    pub fn write_code(&mut self, byte: Byte) {
        self.code.push(byte);
    }
    pub fn code_size(&self) -> usize {
        self.code.len()
    }
    pub fn read_code(&self, idx: usize) -> Byte {
        self.code[idx]
    }
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
    pub fn read_constant(&self, idx: usize) -> Value {
        self.values[idx]
    }
}
