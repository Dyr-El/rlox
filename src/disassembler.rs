use crate::chunk;

fn dump_simple_instr(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn dump_instruction(chunk: &chunk::Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    let instr_code = chunk.read_code(offset);
    if let Ok(instr) = chunk::OpCode::try_from(instr_code) {
        match instr {
            chunk::OpCode::OpReturn => dump_simple_instr("OP_RETURN", offset),
        }
    } else {
        println!("Unknown opcode {}", instr_code);
        offset + 1
    }
}

pub fn dump_chunk(chunk: &chunk::Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    while offset < chunk.code_size() {
        offset = dump_instruction(chunk, offset);
    }
}
