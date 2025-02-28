use std::process::ExitCode;

mod chunk;
mod disassembler;

pub fn run() -> ExitCode {
    let mut chunk = crate::chunk::Chunk::new();
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpReturn));
    disassembler::dump_chunk(&chunk, "test chunk");
    ExitCode::SUCCESS
}
