use std::process::ExitCode;

mod chunk;
mod disassembler;
mod values;

pub fn run() -> ExitCode {
    let mut chunk = crate::chunk::Chunk::new();
    let constant = chunk.add_constant(values::Value::from(1.2));
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpReturn), 122);
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpReturn), 122);
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpConstant), 123);
    chunk.write_code(chunk::Byte::from(constant), 123);
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpReturn), 123);
    disassembler::dump_chunk(&chunk, "test chunk");
    ExitCode::SUCCESS
}
