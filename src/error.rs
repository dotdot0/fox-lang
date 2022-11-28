#![allow(unused, dead_code)]
use crate::token::Token;

#[derive(Debug)]
pub struct LoxError{
  pub line: usize,
  pub message: String
}

impl LoxError{
  pub fn error(line: usize, message: String) -> LoxError{
    LoxError { line, message }
  }

  pub fn report(&self, loc: &String) {
    eprintln!("[ ERROR ] line: {} => {}", self.line, self.message)
  }
}
