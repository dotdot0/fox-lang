#![allow(dead_code, unused)]
use crate::token_type::TokenType;
use crate::token::{Object, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt{
  Print{
    value: Box<Expr>
  },

  Expression{
    value: Box<Expr>
  },

  Var{
    name: Token,
    initializer: Box<Expr>
  }
}


#[derive(Clone, PartialEq, Debug)]
pub enum Expr{
  
  Binary{
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>
  },

  Unary{
    operator: Token,
    right: Box<Expr>
  },

  Literal{
    value: Option<Object>
  },

  Grouping{
    expression: Box<Expr>
  },

  Variable{
    name: Token
  }


}

