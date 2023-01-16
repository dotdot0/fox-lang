#![allow(unused)]
//Modules
mod token_type;
mod token;
mod scanner;
mod error;
mod ast;
mod parser;
mod interpreter;
mod expr;
mod astprinter;
mod stmt;
mod vm;

//use error::LoxError;
//Imports
use scanner::Scanner;
use parser::Parser;
use std::env::args;
use std::io::{self, Write, Read};
use std::fs::File;
use crate::vm::debug::disassemble_chunk;

fn main() {

  // let mut chunk = vm::chunk::Chunk::init();

  // let string = String::new();

  // chunk.write_chunk(vm::chunk::Operation::Constant);

  // let constant = chunk.add_constant(1.2);
  // chunk.write_chunk(vm::chunk::Operation::Return, 123);
  // chunk.write_chunk(vm::chunk::Operation::Constant, 124);
  // chunk.write_chunk_u8(constant as u8, 124);

  // let con = chunk.add_constant(1.42);
  // chunk.write_chunk(vm::chunk::Operation::Constant, 125);
  // chunk.write_chunk_u8(con as u8, 125);

  // let con2 = chunk.add_constant(1.50);
  // chunk.write_chunk(vm::chunk::Operation::Constant, 126);
  // chunk.write_chunk_u8(con2 as u8, 126);
  // disassemble_chunk(&chunk);
   
    let args: Vec<String> = args().collect();

    if args.len() > 2{
      println!("Usage: rlox [script]");
      std::process::exit(64)
    }
    else if args.len() == 2{
      run_file(&args[1])
    }
    else{
      run_prompt()
    }
}

fn run_file(file: &String){ 
  let mut file: File = File::open(file).unwrap();
  let mut file_content: String = String::new();
  file.read_to_string(&mut file_content).unwrap();
  run(file_content)
}

fn run_prompt(){

    println!(r#" 
                           ฅ^•ﻌ•^ฅ
    =======================================================
                  Welcome to fox-lang repl! 
    =======================================================    
                            |\__/|
                           /     \
                          /_.~ ~,_\   
                             \@/
  
    "#);
    
  loop {
    
    print!(">>>");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read code!");
    run(input)
  }
}

fn run(source: String) {
  let mut scanner: Scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens().unwrap(); 
  let mut parser = Parser::new(tokens.clone());
  let stmts = parser.parse();
  if stmts.is_ok(){
    println!("{:#?}", stmts); 
  }
  else{
    let error = stmts.err().unwrap();
    error.report(&String::from(""));
  }
}
