use crate::{ast::{Expr, Stmt}, token::Token};


#[derive(Debug, Clone, PartialEq)]
pub struct Print{
  pub value: Box<Expr>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression{
  pub value: Box<Expr>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var{
  pub name: Token,
  pub initializer: Box<Expr>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block{
  pub statements: Vec<Box<Stmt>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct If{
  pub condition: Box<Expr>,
  pub then_branch: Box<Stmt>,
  pub else_branch: Box<Option<Stmt>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct While{
  pub condition: Box<Expr>,
  pub body: Box<Stmt>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function{
  pub name: Token,
  pub params: Vec<Token>,
  pub body: Vec<Box<Stmt>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return{
  pub keyword: Token,
  pub value: Box<Expr>
}