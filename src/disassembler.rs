use crate::chunk;

pub fn dump_chunk(chunk: &chunk::Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset: usize = 0;
    while offset < chunk.code_size() {
        offset = dump_instruction(chunk, offset);
    }
}

fn dump_instruction(chunk: &chunk::Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.read_line(offset) == chunk.read_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.read_line(offset))
    }
    let instr_code = chunk.read_code(offset);
    if let Ok(instr) = chunk::OpCode::try_from(instr_code) {
        match instr {
            chunk::OpCode::OpReturn => dump_simple_instr("OP_RETURN", offset),
            chunk::OpCode::OpConstantLong => dump_long_constant_instr("OP_CONSTANT_LONG", chunk, offset),
            chunk::OpCode::OpConstant => dump_constant_instr("OP_CONSTANT", chunk, offset),
        }
    } else {
        println!("Unknown opcode {}", instr_code);
        offset + 1
    }
}

fn dump_simple_instr(name: &str, offset: usize) -> usize {
    println!("{:16}", name);
    offset + 1
}

fn dump_constant_instr(name: &str, chunk: &chunk::Chunk, offset: usize) -> usize {
    let constant = usize::from(chunk.read_code(offset + 1));
    print!("{:16} {:8} '", name, constant);
    chunk.read_constant(constant).print();
    println!("'");
    offset + 2
}

fn dump_long_constant_instr(name: &str, chunk: &chunk::Chunk, offset: usize) -> usize {
    let c1 = chunk.read_code(offset + 1);
    let c2 = chunk.read_code(offset + 2);
    let c3 = chunk.read_code(offset + 3);
    let idx = usize::from(c1) << 16 | usize::from(c2) << 8 | usize::from(c3);
    print!("{:16} {:8} '", name, idx);
    chunk.read_constant(usize::from(idx)).print();
    println!("'");
    offset + 4
}
