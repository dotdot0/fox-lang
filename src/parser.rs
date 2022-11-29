#![allow(unused)]


use crate::{token::Token, ast::{Expr, Stmt}, token_type::{TokenType, self}, token::Object, error::LoxError, expr::{Literal, Assigment, Variable, Logical, Binary, Unary, Call, Grouping}, stmt::{Block, If, Print, Return, Var, While, Expression, Function}};

pub struct Parser{
  tokens: Vec<Token>,
  current: usize
}

type ParseError = Result<Expr, LoxError>;

/* 
program        → declaration* EOF ;
declaration    → funDecl
               | varDecl
               | statement ;
funDecl        → "fn" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
statement      → exprStmt
               | ifStmt
               | printStmt
               | block 
               | whileStmt
               | returnStmt ;
returnStmt     → "return" expression? ";" ;
whileStmt      → "while" "(" expression ")" statement ;
ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;
block          → "{" declaration* "}" ;
varDecl        → "let" IDENTIFIER ( "=" expression )? ";" ;
exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;
expression     → equality 
               | assigment ;
assigment      → IDENTIFIER "=" assigment 
               | logic_or;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
call           → primary ( "(" arguments? ")" )* ;
arguments      → expression ( "," expression )* ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" | IDENTIFIER;
                  ^^^^^^^^^^^^^^^^ -> Grouping
*/

impl Parser{
  pub fn new(tokens: Vec<Token>) -> Self{
    Self { tokens, current: 0 }
  }

  pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError>{
    let mut statements: Vec<Stmt> = Vec::new();
    while !self.is_at_end() {
        statements.push(self.declaration()?)
        // println!("{:#?}", self.declaration().unwrap())
    }
    Ok(statements)
  }

  fn declaration(&mut self) -> Result<Stmt, LoxError>{
    if self.is_match(vec![TokenType::VAR]){
      self.var_declaration()
    }
    else if self.is_match(vec![TokenType::Fun]){
      self.function("function")
    }
    else{
      self.statement()
    }
  }

  fn statement(&mut self) -> Result<Stmt, LoxError>{
    if self.is_match(vec![TokenType::Print]){
      return Ok(self.print_statement()?);
    }
    else if self.is_match(vec![TokenType::LeftBrace]){
      return Ok(Stmt::Block(Block{ statements: self.block()? }));
    }
    else if self.is_match(vec![TokenType::If]){
      return Ok(self.if_statement()?);
    }
    else if self.is_match(vec![TokenType::WHILE]){
      return self.while_statement();
    }
    else if self.is_match(vec![TokenType::RETRUN]){
      return self.return_statement();
    }
    Ok(self.expression_statement()?)
  }

  fn if_statement(&mut self) -> Result<Stmt, LoxError>{
    let mut condition: Option<Expr> = None;
    if self.consume(TokenType::LeftParen, String::from("(")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from("Expect '(' after 'if'.")));
    }
    else{
      condition = Some(self.expression()?);
      if !self.consume(TokenType::RightParen, String::from("Expect ')' after if condition.")).is_ok(){
        return Err(LoxError::error(self.previous().line, String::from("Expect ')' after if condition.")));
      }
    }
    let then_branch = self.statement()?;
    let mut else_branch = None;
    if self.is_match(vec![TokenType::Else]){
      else_branch = Some(self.statement()?)
    }
    Ok(Stmt::If(If{ condition: Box::new(condition.unwrap()), then_branch: Box::new(then_branch), else_branch: Box::new(else_branch) }))
  }

  fn block(&mut self) -> Result<Vec<Box<Stmt>>, LoxError>{
    let mut statements: Vec<Box<Stmt>> = Vec::new();

    while !self.check(TokenType::RightBrace) && !self.is_at_end(){
      statements.push(Box::new(self.declaration()?))
    }

    if self.consume(TokenType::RightBrace, String::from("}")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from("Expect '}' after block.")));
    }
    Ok(statements)
  }

  fn print_statement(&mut self) -> Result<Stmt, LoxError>{
    //value can be any of the valid expression
    let value = self.expression()?;
    if self.consume(TokenType::Semicolon, String::from("Expect ; after the value")).is_err(){
      Err(LoxError::error(self.previous().line, String::from("Expect ; after the expression")))
    }else{
      Ok(Stmt::Print(Print{ value: Box::new(value) }))
    }
  }

  fn return_statement(&mut self) -> Result<Stmt, LoxError>{
    let keyword = self.previous();
    let mut value: Option<Expr> = None;
    if !self.check(TokenType::Semicolon){
      value = Some(self.expression()?)
    }

    if self.consume(TokenType::Semicolon, String::from(";")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from("Expect ';' after return statement.")));
    }
    Ok(Stmt::Return(Return{ keyword, value: Box::new(value.unwrap()) }))
  }

  //Parse Variable decl statement
  fn var_declaration(&mut self) -> Result<Stmt, LoxError>{
    let mut initializer: Option<Expr> = None;
    let name = self.consume(TokenType::Identifier, String::from("var name"));
    
    //Check if a ident in provided
    if name.is_err(){
      Err(LoxError::error(self.previous().line, String::from("Expect variable name.")))
    }

    else if self.is_match(vec![TokenType::Equal]){
      //Check if a value is provided after the '='
      if self.is_match(vec![TokenType::Semicolon]){
        return Err(LoxError::error(self.previous().line, "Expect a value after '='.".to_owned()));
      }
      initializer = Some(self.expression()?);
      if self.consume(TokenType::Semicolon, String::from("Expect ';' after variable declaration.")).is_err(){
        return Err(LoxError::error(self.previous().line, String::from("Expect ';' after variable declaration")));
      }else{
        return Ok(Stmt::Var(Var{ name: name.unwrap(), initializer: Box::new(initializer.unwrap()) }));
      }
    }

    //If '=' not found treat the value ad nil
    else {
      Ok(Stmt::Var(Var{ name: self.previous(), initializer: Box::new(Expr::Literal(Literal{ value: Some(Object::Nil) }))}))
    }
  }

  fn while_statement(&mut self) -> Result<Stmt, LoxError>{
    let mut expr: Option<Expr> = None;
    let mut stmt: Option<Stmt> = None;
    if self.consume(TokenType::LeftParen, String::from("(")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from("Expect '(' after 'while'.")));
    }
    else{
      expr = Some(self.expression()?);
    }
    if self.consume(TokenType::RightParen, String::from(")")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from("Expect ')' after condition.")));
    }
    else{
      stmt = Some(self.statement()?);
    }
    Ok(Stmt::While(While{ condition: Box::new(expr.unwrap()), body: Box::new(stmt.unwrap()) }))
  }

  fn expression_statement(&mut self) -> Result<Stmt, LoxError>{
    let value = self.expression()?;
    if self.consume(TokenType::Semicolon, String::from("Expect ; after the value")).is_err(){
      Err(LoxError::error(self.previous().line, String::from("Expect ; after the expression")))
    }
    else{
      Ok(Stmt::Expression(Expression{ value: Box::new(value) }))
    }
  }

  fn function(&mut self, kind: &str) -> Result<Stmt, LoxError>{
    let name = self.consume(TokenType::Identifier, String::from("Ident"));

    //Check if function name is provided
    if name.is_err(){
      return Err(LoxError::error(self.previous().line, String::from(format!("Expect {kind} name."))));
    }
    else if self.consume(TokenType::LeftParen, String::from("(")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from(format!("Expect '(' after {kind} name."))));
    }
    let mut params: Vec<Token> = Vec::new();
    if !self.check(TokenType::RightParen){
      
      //First consume the first argument then enter the loop of ','
      params.push(self.consume(TokenType::Identifier, String::from("Ident"))?);
      while self.is_match(vec![TokenType::Comma]) {
        if self.check(TokenType::Identifier){
          if params.len() > 255{
            return Err(LoxError { line: self.peek().line, message: String::from("Can't have more than 255 parameters") });
          }
          params.push(self.consume(TokenType::Identifier, String::from("Ident"))?)
      
          }else{
            return Err(LoxError::error(self.previous().line, "Expected a param after ','".to_string()));
          }  
        }
    }
    if self.consume(TokenType::RightParen, String::from(")")).is_err() {
      return Err(LoxError::error(self.previous().line, String::from("Expect ')' after parameters.")));
    }
    else if self.consume(TokenType::LeftBrace, String::from("{")).is_err(){
      return Err(LoxError::error(self.previous().line, String::from(format!("Expect before {kind} body."))));
    }
    let body: Vec<Box<Stmt>> = self.block()?;
    Ok(Stmt::Function(Function{ name: name.unwrap(), params, body }))
  }

  fn expression(&mut self) -> ParseError{
    self.assigment()
  }
  
  fn assigment(&mut self) -> ParseError{
    let mut expr = self.or()?;

    if self.is_match(vec![TokenType::Equal]){
      let equal = self.previous();
      let value = self.assigment()?;

      match expr {
          Expr::Variable(Variable{ name }) => {
            return Ok(Expr::Assigment(Assigment{ name, value: Box::new(value) }));
          },
          _ => {
            return Err(LoxError::error(self.previous().line, String::from("Invalid assigment target.")))}
      }
    }

    Ok(expr)
  }

  fn or(&mut self) -> ParseError{
    let mut expr = self.and()?;

    while self.is_match(vec![TokenType::Or]) {
      let operator = self.previous();
      let right = self.and()?;
      expr = Expr::Logical(Logical{ left: Box::new(expr), operator, right: Box::new(right) })
    }
    Ok(expr)
  }

  fn and(&mut self) -> ParseError{
    let mut expr = self.equality()?;

    while self.is_match(vec![TokenType::And]) {
      let operator = self.previous();
      let right = self.equality()?;
      expr = Expr::Logical(Logical{ left: Box::new(expr), operator, right: Box::new(right) })
    }
    Ok(expr)
  }

  fn equality(&mut self) -> ParseError{
    let mut expr = self.comparison()?;

    while self.is_match(vec![TokenType::Bang_Equal, TokenType::Equal_Equal]){
      let operator = self.previous();
      let right = self.comparison()?;
      expr = Expr::Binary(Binary{ left: Box::new(expr), operator, right: Box::new(right) })
    }
    Ok(expr)
  }
 
  fn comparison(&mut self) -> ParseError{
    let mut expr = self.term()?;

    while self.is_match(vec![TokenType::Greater_Equal, TokenType::Greater, TokenType::Less_Equal, TokenType::Less]) {
        let operator = self.previous();
        let right = self.term()?;
        expr = Expr::Binary(Binary{ left: Box::new(expr), operator, right: Box::new(right) })
    }
    Ok(expr)
  }

  fn term(&mut self) -> ParseError{
    let mut expr = self.factor()?;

    while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
        let operator = self.previous();
        let right = self.factor()?;
        expr = Expr::Binary(Binary{ left: Box::new(expr), operator, right: Box::new(right) })
    }
    Ok(expr)
  }

  fn factor(&mut self) -> ParseError{
    let mut expr = self.unary()?;

    while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
        let operator = self.previous();
        let right = self.unary()?;
        expr = Expr::Binary(Binary{ left: Box::new(expr), operator, right: Box::new(right) })
    }
    Ok(expr)
  }

  fn unary(&mut self) -> ParseError{
    if self.is_match(vec![TokenType::Minus, TokenType::Bang]){
      let operator = self.previous();
      let expr = self.unary()?;
      return Ok(Expr::Unary(Unary{operator, right: Box::new(expr) }));
    }
    self.call()
  }

  fn finish_call(&mut self, callee: Expr) -> ParseError{
    let mut arguments: Vec<Box<Expr>> = Vec::new();
   
    //Until we don't reach a ')' keep adding arguments 
    if !self.check(TokenType::RightParen){
      arguments.push(Box::new(self.expression()?));
      
      //Loop gonna run until it finds ',' as the next token
      while self.is_match(vec![TokenType::Comma]) {
          if arguments.len() >= 255{
            return Err(LoxError::error(self.previous().line, String::from("Can't have more than 255 arguments.")));
          }
          arguments.push(Box::new(self.expression()?))
      }
    }

    let paren = self.consume(TokenType::RightParen, String::from(")"));
    if paren.is_err(){
      return Err(LoxError::error(self.previous().line, String::from("Expect ')' after arguments.")));
    }

    Ok(Expr::Call(Call{ callee: Box::new(callee), paren: paren.unwrap(), arguments }))
  }

  //Cause function call starts with a indent so it's high in precedence
  fn call(&mut self) -> ParseError{
    let mut expr = self.primary()?;

    loop{
        if self.is_match(vec![TokenType::LeftParen]){
          expr = self.finish_call(expr)?;
        }else{
          break;
        }
    }
    Ok(expr)
  }

  fn primary(&mut self) -> ParseError{
    if self.is_match(vec![TokenType::False]) { 
       Ok(Expr::Literal(Literal{ value: Some(Object::Bool(false)) }))
    }

    else if self.is_match(vec![TokenType::True]){
       Ok(Expr::Literal(Literal{ value: Some(Object::Bool(true)) }))
    }

    else if self.is_match(vec![TokenType::Nil]){
       Ok(Expr::Literal(Literal{ value: Some(Object::Nil) }))
    }

    else if self.is_match(vec![TokenType::STRING, TokenType::Number]){
       Ok(Expr::Literal(Literal{ value: self.previous().literal }))
    }

    else if self.is_match(vec![TokenType::LeftParen]){
      let mut expr = self.expression()?;

      if self.consume(TokenType::RightParen, String::from("Expect ) after expression.")).is_err(){
        Err(LoxError::error(self.previous().line, String::from("Expect ) after expression.")))
      }
      else{
        Ok(Expr::Grouping(Grouping{ expression: Box::new(expr) }))
      }
    }else if self.is_match(vec![TokenType::Identifier]){
      Ok(Expr::Variable(Variable{ name: self.previous() }))
    }
    else {
        Ok(Expr::Literal(Literal{ value: None }))
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
      Err(LoxError::error(self.previous().line, String::from("Expect ) at the end ")))
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
