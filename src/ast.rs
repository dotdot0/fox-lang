#![allow(dead_code, unused)]
use crate::expr::{ExprVisitor, Binary, Unary, Literal, Grouping, Variable, Assigment, Logical, Call};
use crate::stmt::{Print, Expression, Var, Block, If, While, Function, Return};
use crate::token_type::TokenType;
use crate::token::{Object, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt{
  Print(Print),

  Expression(Expression),

  Var(Var),

  Block(Block),

  If(If),

  While(While),

  Function(Function),

  Return(Return)
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
