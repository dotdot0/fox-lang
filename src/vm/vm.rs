use super::chunk::{Chunk, u8_to_operation, Operation};

pub struct VM{
    chunk: Chunk,
    ip: usize
}

enum InterpretResult{
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

       loop{
           instruction = self.chunk.code[self.ip];
           let operation = u8_to_operation(instruction);
           match operation{
               Operation::Return => break InterpretResult::InterpretOk,
               _ => todo!()
           }
       }
    }
}
