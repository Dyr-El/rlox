#[derive(PartialEq, Clone, Copy)]
pub struct Byte(u8);

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum OpCode {
    OpReturn,
}

impl From<OpCode> for Byte {
    fn from(m: OpCode) -> Byte {
        Byte(m as u8)
    }
}

impl TryFrom<Byte> for OpCode {
    type Error = ();

    fn try_from(byte: Byte) -> Result<Self, Self::Error> {
        const OP_RETURN_BYTE: Byte = Byte(OpCode::OpReturn as u8);
        match byte {
            OP_RETURN_BYTE => Ok(OpCode::OpReturn),
            _ => Err(()),
        }
    }
}

pub struct Chunk {
    code: Vec<Byte>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![] }
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
}
