use std::process::ExitCode;

mod chunk;
mod disassembler;
mod values;

pub fn run() -> ExitCode {
    let mut chunk = crate::chunk::Chunk::new();
    let constant = chunk.add_constant(values::Value::from(1.2));
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpConstant));
    chunk.write_code(chunk::Byte::from(constant));
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpReturn));
    disassembler::dump_chunk(&chunk, "test chunk");
    ExitCode::SUCCESS
}
