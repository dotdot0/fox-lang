use crate::vm::chunk::{Chunk, Operation, u8_to_operation};

pub fn disassemble_chunk(chunk: &Chunk){
    println!("==BYTECODE==");

    let mut offset: usize = 0;

    while offset < chunk.code.len(){
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{ 
    print!("{:#04} ", offset);
    let instruction: Operation = u8_to_operation(chunk.code[offset]);

    match instruction{
        Operation::Return => return simple_instruction("RETURN".to_string(), offset),
        _ => println!("{}", "INVALID INSTRUCTION".to_string())
    }

    offset + 1 
}

fn simple_instruction(name: String, offset: usize) -> usize{
    print!("{name}\n");
    offset + 1
}
