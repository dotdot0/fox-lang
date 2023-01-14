use super::chunk::{Chunk, u8_to_operation, Operation, Value};
use crate::vm::debug::print_value;

pub struct VM{
    chunk: Chunk,
    ip: usize
}

pub enum InterpretResult{
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError
}

impl VM{
    pub fn init_vm(chunk: Chunk) -> Self{
        Self {
            chunk,
            ip: 0
        }
    }

    pub fn interpret(&mut self) -> InterpretResult{
        self.run()
    }

    fn run(&mut self) -> InterpretResult{
       let mut instruction = 0;

       /*
        #[Run]
        Read the byte at the current ip
        executes it return InterpretResult
        increment the ip
        */

       loop{
           instruction = self.chunk.code[self.ip];
           let operation = u8_to_operation(instruction);
           match operation{
               Operation::Return => return InterpretResult::InterpretOk,
               Operation::Constant => {
                   let constant: Value = self.chunk.constants[self.ip];
                   print_value(constant);
                   print!("\n");
                   break InterpretResult::InterpretOk;
               }
               _ => todo!()
           }
           self.ip += 1;
       }
    }
}
