use crate::vm::chunk::{Chunk, Operation, u8_to_operation, Value};

pub fn disassemble_chunk(chunk: &Chunk){
    println!("==BYTECODE==");

    let mut offset: usize = 0;

    while offset < chunk.code.len(){
        offset = disassemble_instruction(chunk, offset);
    }
    
    println!("CONSTANT POOL: {:?}", chunk.constants);

    println!("==END==");
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{ 
    print!("{:#04} ", offset);

    if offset > 0 && 
    chunk.lines.get(offset).unwrap() == chunk.lines.get(offset - 1).unwrap(){
        print!("   | ");
    }
    else{
        print!("{:#04} ", chunk.lines.get(offset).unwrap());
    }
    let instruction: Operation = u8_to_operation(chunk.code[offset]);

    match instruction{
        Operation::Return => return simple_instruction("OP_RETURN".to_string(), offset),
        Operation::Constant => return constant_instruction("OP_CONSTANT".to_string(), chunk, offset), 
        _ => println!("INVALID INSTRUCTION {:?}", instruction)
    }

    offset + 1 
}

fn simple_instruction(name: String, offset: usize) -> usize{
    print!("{name}\n");
    offset + 1
}

fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize{
    let constant = chunk.code[offset + 1];
    print!("{} {:#04} ", name, constant);
    print_value(chunk.constants[constant as usize]);
    print!("\n");
    offset + 2
}

pub fn print_value(value: Value){
    print!("{}", value)
}
