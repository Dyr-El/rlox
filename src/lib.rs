use std::process::ExitCode;

mod chunk;
mod disassembler;
mod values;

pub fn run() -> ExitCode {
    let mut chunk = crate::chunk::Chunk::new();
    for idx in 1..1000 {
        chunk.write_const(values::Value::from((idx as f64) / 4.0), idx / 4);
    }
    chunk.write_code(chunk::Byte::from(chunk::OpCode::OpReturn), 255);
    disassembler::dump_chunk(&chunk, "test chunk");
    ExitCode::SUCCESS
}
