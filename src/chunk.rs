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
    OpNil,
    OpTrue,
    OpFalse,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
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
        const OP_NIL_BYTE: Byte = Byte(OpCode::OpNil as u8);
        const OP_TRUE_BYTE: Byte = Byte(OpCode::OpTrue as u8);
        const OP_FALSE_BYTE: Byte = Byte(OpCode::OpFalse as u8);
        const OP_NEGATE_BYTE: Byte = Byte(OpCode::OpNegate as u8);
        const OP_ADD_BYTE: Byte = Byte(OpCode::OpAdd as u8);
        const OP_SUBTRACT_BYTE: Byte = Byte(OpCode::OpSubtract as u8);
        const OP_MULTIPLY_BYTE: Byte = Byte(OpCode::OpMultiply as u8);
        const OP_DIVIDE_BYTE: Byte = Byte(OpCode::OpDivide as u8);
        const OP_RETURN_BYTE: Byte = Byte(OpCode::OpReturn as u8);
        const OP_NOT_BYTE: Byte = Byte(OpCode::OpNot as u8);
        match byte {
            OP_CONSTANT_BYTE => Ok(OpCode::OpConstant),
            OP_CONSTANT_LONG_BYTE => Ok(OpCode::OpConstantLong),
            OP_NIL_BYTE => Ok(OpCode::OpNil),
            OP_TRUE_BYTE => Ok(OpCode::OpTrue),
            OP_FALSE_BYTE => Ok(OpCode::OpFalse),
            OP_NEGATE_BYTE => Ok(OpCode::OpNegate),
            OP_ADD_BYTE => Ok(OpCode::OpAdd),
            OP_SUBTRACT_BYTE => Ok(OpCode::OpSubtract),
            OP_MULTIPLY_BYTE => Ok(OpCode::OpMultiply),
            OP_DIVIDE_BYTE => Ok(OpCode::OpDivide),
            OP_RETURN_BYTE => Ok(OpCode::OpReturn),
            OP_NOT_BYTE => Ok(OpCode::OpNot),
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
    #[cfg(any(feature = "dumpChunk"))]
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
