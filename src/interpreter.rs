use std::borrow::Borrow;

use crate::{expr::ExprVisitor, token::Object, ast::{Stmt, Expr}, error::LoxError};

struct Interpreter{}
  
impl Interpreter{
  pub fn new() -> Self{
    Self {  }
  }

  fn interpreter(&mut self, statements: &Vec<Box<Stmt>>) -> Result<(), LoxError>{
    for s in statements{
      match s.borrow() {
          Stmt::Expression { value } => {
            match value {
                Expr::Literal { value } => 
            }
          }
          _ => todo!()
      }
    }
    Ok(())
  }
}