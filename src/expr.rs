#![allow(unused)]
use crate::{ast::Expr, token::{Token, Object}};

#[derive(Debug, PartialEq, Clone)]
pub struct Binary{
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Unary{
    pub operator: Token,
    pub right: Box<Expr>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal{
    pub value: Option<Object>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping{
    pub expression: Box<Expr>
} 

#[derive(Debug, PartialEq, Clone)]
pub struct Variable{
    pub name: Token
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assigment{
    pub name: Token,
    pub value: Box<Expr>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Logical{
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call{
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Box<Expr>>
}


impl Literal{
  pub fn interpret(&mut self) -> String{
    self.value.as_ref().unwrap().value()
  }
}