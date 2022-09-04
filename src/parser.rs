use crate::token::Token;
use crate::ast::Expr;

struct Parser{
  tokens: Vec<Token>,
  current: usize
}

impl Parser {
  fn new(tokens: Vec<Token>) -> Parser{
    Parser { tokens, current: 0 }
  }

  fn equality(&mut self) -> Expr{
    let mut expr: Expr = self.comparison();

    while 
  }

  fn comparison(&self){}

  fn is_match(&self, token: Token) -> bool{
    if self.tokens.get(self.current)
  }

  fn advance(&mut self) -> Token{
    let tok = self.tokens.get(self.current).unwrap().clone();
    self.current += 1;
    tok
  }
}