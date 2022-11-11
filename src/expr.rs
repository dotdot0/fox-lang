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


pub trait ExprVisitor<T> {
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
    fn visit_binary_expr(&self, expr: &Binary) -> T;
}

impl Binary{
  fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T{
    visitor.visit_binary_expr(self)
  }
}

impl Literal{
  fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T{
    visitor.visit_literal_expr(self)
  }
}

impl Grouping{
  fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T{
    visitor.visit_grouping_expr(self)
  }
}

impl Unary{
  fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T{
    visitor.visit_unary_expr(self)
  }
}