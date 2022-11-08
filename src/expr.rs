use crate::{ast::Expr, token::{Token, Object}};

pub struct Binary{
  left: Box<Expr>,
  operator: Token,
  right: Box<Expr>
}

pub struct Unary{
  operator: Token,
  right: Box<Expr>
}

pub struct Literal{
  value: Option<Object>
}

  



pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self) -> T;
    fn visit_grouping_expr(&mut self) -> T;
    fn visit_unary_expr(&mut self) -> T;
    fn visit_binary_expr(&mut self) -> T;
}