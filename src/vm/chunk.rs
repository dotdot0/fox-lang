#![allow(unused)]

type Value = f32;

#[derive(Debug, Clone, Copy)]
pub enum Operation{
    Return,
    Constant
}

pub struct Chunk{
    code: Vec<Operation>,
    /*
    A way to store const that will result in a value at runtime
    1 + 2; -> 3[But at runtime] Interpreter needs to store 1, 2
    sort of instructions that 'Produce a value'
    */
    constants: Vec<Value>
}

impl Chunk{
    pub fn new() -> Self{
        Self{
            code: vec![],
            constants: vec![]
        }
    }

    pub fn write_chunk(&mut self, op: Operation){
        self.code.push(op);
    }

    pub fn add_constant(&mut self, value: Value) -> i32{
        self.constants.push(value);
        get_index(&self.constants, value)
    }

    pub fn disassemble_chunk(&self, name: &str){

        let mut offset = 0;

        println!("== {name} ==");

        self.code.iter().for_each(|c| {
            offset = self.disassemble_instruction(offset);
        })
    }

    fn disassemble_instruction(&self, offset: i32) -> i32{
        print!("{:#04} ", offset);

        let instruction = self.code[offset as usize];
        
        match instruction {
            Operation::Return => return self.simple_instruction("RETURN", offset),
            _ => println!("Invalid Chunk")
        }

        offset + 1
    }

    fn simple_instruction(&self, name: &str, offset: i32) -> i32{
        print!("{name}");
        offset + 1
    }
}


fn get_index<T: PartialEq>(vec: &Vec<T>, x: T) -> i32{
    let index = vec.iter().position(|element| element == &x).unwrap();
    index as i32
}