#![allow(unused)]
use crate::{token::Token, ast::Expr, token_type::TokenType, token::Object, error::ParseError};

pub struct Parser{
  tokens: Vec<Token>,
  current: usize
}

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;


impl Parser{
  pub fn new(tokens: Vec<Token>) -> Self{
    Self { tokens, current: 0 }
  }

  pub fn parse(&mut self) -> Expr{
    self.expression()
  }

  fn expression(&mut self) -> Expr{
    self.equality()
  }

  fn equality(&mut self) -> Expr{
    let mut expr = self.comparison();

    while self.is_match(vec![TokenType::Bang_Equal, TokenType::Equal_Equal]){
      let operator = self.previous();
      let right = self.comparison();
      expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    expr
  }
 
  fn comparison(&mut self) -> Expr{
    let mut expr = self.term();

    while self.is_match(vec![TokenType::Greater_Equal, TokenType::Greater, TokenType::Less_Equal, TokenType::Less]) {
        let operator = self.previous();
        let right = self.term();
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    expr
  }

  fn term(&mut self) -> Expr{
    let mut expr = self.factor();

    while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
        let operator = self.previous();
        let right = self.factor();
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    expr
  }

  fn factor(&mut self) -> Expr{
    let mut expr = self.unary();

    while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
        let operator = self.previous();
        let right = self.unary();
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    expr
  }

  fn unary(&mut self) -> Expr{
    if self.is_match(vec![TokenType::Minus, TokenType::Bang]){
      let operator = self.previous();
      let expr = self.unary();
      return Expr::Unary { operator, right: Box::new(expr) };
    }
    self.primary()
  }

  fn primary(&mut self) -> Expr{
    if self.is_match(vec![TokenType::False]) { 
      return Expr::Literal { value: Some(Object::Bool(false)) };
    }

    else if self.is_match(vec![TokenType::True]){
      return Expr::Literal { value: Some(Object::Bool(true)) };
    }

    else if self.is_match(vec![TokenType::Nil]){
      return Expr::Literal { value: Some(Object::Nil) };
    }

    else if self.is_match(vec![TokenType::STRING, TokenType::Number]){
      return Expr::Literal { value: self.previous().literal };
    }

    else if self.is_match(vec![TokenType::LeftParen]){
      let mut expr = self.expression();

      self.consume(TokenType::RightParen, String::from("Expect ) after expression."));

      Expr::Grouping { expression: Box::new(expr) }
    }
    
    else{
      Expr::Literal { value: None }
    }
  }

  fn is_match(&mut self, tokens: Vec<TokenType>) -> bool{
    for &t in tokens.iter(){
      if self.check(t){
        self.advance();
        return true;
      }
    }
    false
  }

  //For Error
  fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, ParseError>{
    if self.check(ttype){
      Ok(self.advance())
    }
    else {
      Err(ParseError{tok: self.peek(), message})
    }
  }

  fn check(&self, ttype: TokenType) -> bool{
    if self.is_at_end(){
      false
    }
    else{
      self.peek().ttype == ttype
    }
  }

  fn advance(&mut self) -> Token{
    if !self.is_at_end(){ 
      self.current +=1 
    }
    self.previous()
  }

  fn is_at_end(&self) -> bool{
    self.peek().ttype == TokenType::EOF
  }

  fn peek(&self) -> Token{
    self.tokens.get(self.current).unwrap().clone()
  }

  fn previous(&self) -> Token{
    self.tokens.get(self.current - 1).unwrap().clone()
  }
}
