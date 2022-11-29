#![allow(unused, dead_code)]
use crate::token::Token;

#[derive(Debug)]
pub struct LoxError{
  pub line: usize,
  pub message: String,
  pub column: usize
}

impl LoxError{
  pub fn error(line: usize, column: usize,message: String) -> LoxError{
    LoxError { line, column, message }
  }

  pub fn report(&self, loc: &String) {
    eprintln!("[ ERROR ] line[{}: {}] => {}", self.line, self.column, self.message)
  }
}
