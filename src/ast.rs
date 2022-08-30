#![allow(dead_code, unused)]
use crate::token_type::{TokenType, self};
use crate::token::Object;

pub enum Expr{
  
  Binary{
    left: Box<Expr>,
    operator: TokenType,
    right: Box<Expr>
  },

  Unary{
    operator: TokenType,
    right: Box<Expr>
  },

  Literal{
    value: Option<Object>
  },

  Grouping{
    expression: Box<Expr>
  }


}