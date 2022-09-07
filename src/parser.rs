use crate::token::Token;
use crate::ast::Expr;
use crate::token_type::TokenType;

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

    while self.is_match(&[TokenType::Bang_Equal, TokenType::Equal_Equal]){
      let operator = self.comparison();

    }
  }

  fn comparison(&self) -> Expr{}

  fn is_match(&self, types: &[TokenType]) -> bool{
    for &t in types{
      if self.check(t){
        self.advance();
        return true;
      }
    }
    false
  }

  fn advance(&mut self) -> Token{
    if !self.is_at_end(){
      //let tok = self.tokens.get(self.current).unwrap().clone();
      self.current += 1;
      //tok
    }
    self.peek().to_owned()
  }

  fn is_at_end(&self) -> bool{
    let token = self.peek().to_owned();
    token.ttype == TokenType::EOF
  }

  fn peek(&self) -> &Token{
    self.tokens.get(self.current).unwrap()
  }

  fn previous(&self) -> &Token{
    self.tokens.get(self.current - 1).unwrap()
  }

  fn check(&self, ttype: TokenType) -> bool{
    if self.is_at_end(){
      false
    }else {
        self.peek().ttype == ttype
    }
  }

}