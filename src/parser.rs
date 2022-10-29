#![allow(unused)]
use crate::token::{Token, Object::*};
use crate::ast::Expr;
use crate::token_type::TokenType;

struct Parser{
  tokens: Vec<Token>,
  current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self{
      Self{
        tokens,
        current: 0
      }
    }

    fn expression(&self) -> Expr{
      self.equality()
    }

    fn equality(&self) -> Expr{
      let mut expr: Expr = self.comparison();
      while self.is_match(&[TokenType::Bang_Equal, TokenType::Equal_Equal]) {
          let operator = self.prev().clone();
          let right = self.comparison();
          let expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
      }
      expr
    }

    fn comparison(&self) -> Expr{
      let mut expr: Expr = self.term();

      while self.is_match(&[TokenType::Greater, TokenType::Greater_Equal, TokenType::Less, TokenType::Less_Equal]){
        let operator: Token = self.prev().clone();
        let right: Expr = self.term();
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right)}
      }
      expr
    }

    fn term(&self) -> Expr{
      let mut expr: Expr = self.factor();
      while self.is_match(&[TokenType::Plus, TokenType::Minus]) {
          let operator = self.prev().clone();
          let right = self.factor();
          expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
      }
      expr
    }

    fn factor(&self) -> Expr{
      let mut expr: Expr = self.unary();
      while self.is_match(&[TokenType::Star, TokenType::Slash]) {
          let operator = self.prev().clone();
          let right = self.unary(); 
          expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) } 
      }
      expr
    }

    fn unary(&self) -> Expr{
      if self.is_match(&[TokenType::Bang, TokenType::Minus]){
        let operator = self.prev().clone();
        let right = self.unary();
        return Expr::Unary { operator, right: Box::new(right) };
      }

      return self.primary();
    }
    
    fn primary(&self) -> Expr{
      if self.is_match(&[TokenType::False]){ 
        Expr::Literal { value: Some(Bool(false)) }
      }
      else if self.is_match(&[TokenType::True]) {
        Expr::Literal { value: Some(Bool(true))}
      }
      else if self.is_match(&[TokenType::Nil]){
        Expr::Literal { value: Some(Nil) }
      }
      else if self.is_match(&[TokenType::STRING, TokenType::Number]){
        Expr::Literal { value: self.prev().literal.clone() }
      }
      else{
        Expr::Literal { value: None }
      }

    }

    fn is_match(&self, types: &[TokenType]) -> bool{
      for &t in types{
        if self.check(t){
          self.advance();
          return true;
        }
      }
      false
    }

    fn check(&self, ttype: TokenType) -> bool{
      if self.is_at_end(){
        false
      }
      else{
        return self.peek().ttype == ttype;
      }
    }

    fn advance(&self) -> &Token{
      if !self.is_at_end(){
        self.current += 1;
      }
      self.prev()
    }

    fn is_at_end(&self) -> bool{
      if self.peek().ttype == TokenType::EOF{
        true
      }else{
        false
      }
    }

    fn peek(&self) -> &Token{
      self.tokens.iter().nth(self.current).unwrap()
    }

    fn prev(&self) -> &Token{
      self.tokens.iter().nth(self.current - 1).unwrap()
    }
}