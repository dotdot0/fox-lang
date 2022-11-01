#![allow(unused, dead_code)]
use crate::token::Token;

pub struct LoxError{
  pub line: usize,
  pub message: String
}

impl LoxError{
  pub fn error(line: usize, message: String) -> LoxError{
    LoxError { line, message }
  }

  pub fn report(&self, loc: &String) {
    eprintln!("[line {}] Error {}: {}", self.line,loc, self.message)
  }
}

pub struct ParseError{
  pub tok: Token,
  pub message: String
}

impl ParseError{
  pub fn report(&self){
    println!("{:?}, {}, {}", self.tok, self.tok.line, self.message)
  }
}