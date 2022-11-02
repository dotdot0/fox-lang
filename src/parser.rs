#![allow(unused)]
use crate::{token::Token, ast::{Expr, Stmt}, token_type::{TokenType, self}, token::Object, error::LoxError};

pub struct Parser{
  tokens: Vec<Token>,
  current: usize
}

type ParseError = Result<Expr, LoxError>;

// program        → statement* EOF ;

// statement      → exprStmt
//                | printStmt ;

// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
//                   ^^^^^^^^^^^^^^^^ -> Grouping

impl Parser{
  pub fn new(tokens: Vec<Token>) -> Self{
    Self { tokens, current: 0 }
  }

  pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError>{
    let mut statements: Vec<Stmt> = Vec::new();
    while !self.is_at_end() {
        statements.push(self.statement()?)
    }
    Ok(statements)
  }

  fn statement(&mut self) -> Result<Stmt, LoxError>{
    if self.is_match(vec![TokenType::Print]){
      return Ok(self.print_statement()?);
    }
    Ok(self.expression_statement()?)
  }

  fn print_statement(&mut self) -> Result<Stmt, LoxError>{
    let value = self.expression()?;
    if self.consume(TokenType::Semicolon, String::from("Expect ; after the value")).is_err(){
      Err(LoxError::error(self.previous().line, String::from("Expect ; after the expression")))
    }else{
      Ok(Stmt::Print { value: Box::new(value) })
    }
  }

  fn expression_statement(&mut self) -> Result<Stmt, LoxError>{
    let value = self.expression()?;
    if self.consume(TokenType::Semicolon, String::from("Expect ; after the value")).is_err(){
      Err(LoxError::error(self.previous().line, String::from("Expect ; after the expression")))
    }
    else{
      Ok(Stmt::Expression { value: Box::new(value) })
    }
  } 

  fn expression(&mut self) -> ParseError{
    self.equality()
  }

  fn equality(&mut self) -> ParseError{
    let mut expr = self.comparison()?;

    while self.is_match(vec![TokenType::Bang_Equal, TokenType::Equal_Equal]){
      let operator = self.previous();
      let right = self.comparison()?;
      expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    Ok(expr)
  }
 
  fn comparison(&mut self) -> ParseError{
    let mut expr = self.term()?;

    while self.is_match(vec![TokenType::Greater_Equal, TokenType::Greater, TokenType::Less_Equal, TokenType::Less]) {
        let operator = self.previous();
        let right = self.term()?;
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    Ok(expr)
  }

  fn term(&mut self) -> ParseError{
    let mut expr = self.factor()?;

    while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
        let operator = self.previous();
        let right = self.factor()?;
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    Ok(expr)
  }

  fn factor(&mut self) -> ParseError{
    let mut expr = self.unary()?;

    while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
        let operator = self.previous();
        let right = self.unary()?;
        expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
    }
    Ok(expr)
  }

  fn unary(&mut self) -> ParseError{
    if self.is_match(vec![TokenType::Minus, TokenType::Bang]){
      let operator = self.previous();
      let expr = self.unary()?;
      return Ok(Expr::Unary { operator, right: Box::new(expr) });
    }
    self.primary()
  }

  fn primary(&mut self) -> ParseError{
    if self.is_match(vec![TokenType::False]) { 
       Ok(Expr::Literal { value: Some(Object::Bool(false)) })
    }

    else if self.is_match(vec![TokenType::True]){
       Ok( Expr::Literal { value: Some(Object::Bool(true)) })
    }

    else if self.is_match(vec![TokenType::Nil]){
       Ok(Expr::Literal { value: Some(Object::Nil) })
    }

    else if self.is_match(vec![TokenType::STRING, TokenType::Number]){
       Ok(Expr::Literal { value: self.previous().literal })
    }

    else if self.is_match(vec![TokenType::LeftParen]){
      let mut expr = self.expression()?;

      self.consume(TokenType::RightParen, String::from("Expect ) after expression."));

      Ok(Expr::Grouping { expression: Box::new(expr) })
    }else{
      Err(LoxError::error(self.previous().line, String::from("Expect ) after expression")))
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
  fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, LoxError>{
    if self.check(ttype){
      Ok(self.advance())
    }
    else {
      Err(LoxError::error(self.current, String::from("Expect ) at the end ")))
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

  fn had_error(&self,p: ParseError) -> bool{
    p.is_err()
  }
}
