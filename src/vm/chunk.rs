#![allow(unused)]

use std::fmt::{self, write};
use std::convert::From;

pub type Value = f32;

#[derive(Debug, Clone, Copy)]
pub enum Operation{
    Return = 0,
    Constant
}


pub struct Chunk{
    pub code: Vec<u8>,
    /*
    A way to store const that will result in a value at runtime
    1 + 2; -> 3[But at runtime] Interpreter needs to store 1, 2
    sort of instructions that 'Produce a value'
    */
    pub constants: Vec<Value>,
    /* 
    Element at each index of lines is 
    the line number of the instruction 
    at the same index
    */
    pub lines: Vec<i32>
}

impl Chunk{
    pub fn init() -> Self{
        Self{
            code: vec![],
            constants: vec![],
            lines: vec![]
        }
    }

    pub fn write_chunk(&mut self, op: Operation, line: i32){
        self.code.push(op as u8);
        self.lines.push(line);
    }

    //Used to add const index in the instructions
    pub fn write_chunk_u8(&mut self, c: u8, line: i32){
        self.code.push(c);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize{
        self.constants.push(value);
        get_index(&self.constants, value)
    }

}


fn get_index<T: PartialEq>(vec: &Vec<T>, x: T) -> usize{
    let index = vec.iter().position(|element| element == &x).unwrap();
    index
}

pub fn u8_to_operation(byte_code: u8) -> Operation{
    match byte_code{
        0 => Operation::Return,
        1 => Operation::Constant,
        _ => todo!()
    }
}
