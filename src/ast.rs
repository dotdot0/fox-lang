#![allow(dead_code, unused)]
use crate::token_type::TokenType;
use crate::token::{Object, Token};

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

  LetStatement{
    name: TokenType,
    value: Box<Expr>
  }

}