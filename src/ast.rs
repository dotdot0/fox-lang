#![allow(dead_code, unused)]
use crate::expr::{ExprVisitor, Binary, Unary, Literal, Grouping, Variable, Assigment, Logical, Call};
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
  },

  Block{
    statements: Vec<Box<Stmt>>
  },

  Class{
    name: Token,
    methods: Vec<Box<Stmt>>
  },

  If{
    condition: Box<Expr>,
    then_branch: Box<Stmt>,
    else_branch: Box<Option<Stmt>>
  },

  While{
    condition: Box<Expr>,
    body: Box<Stmt>
  },

  Function{
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<Stmt>>
  },

  Return{
    keyword: Token,
    value: Box<Expr>
  }
}


#[derive(Clone, PartialEq, Debug)]
pub enum Expr{
  
  Binary(Binary),

  Unary(Unary),

  Literal(Literal),

  Grouping(Grouping),

  Variable(Variable),

  Assigment(Assigment),

  Logical(Logical),

  Call(Call)

}
