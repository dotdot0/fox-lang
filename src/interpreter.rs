#![allow(unused)]
use crate::ast::{Expr, Stmt};

pub struct Interpreter{
  pub asts: Vec<Stmt>
}

impl Interpreter{
  pub fn new(asts: Vec<Stmt>) -> Self{
    Self { asts }
  }

  pub fn interpret(&self){
    for i in self.asts.iter(){
      println!("{:#?}", i)
    }
  }
}