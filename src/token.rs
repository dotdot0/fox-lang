use crate::token_type::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token{
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize
}

impl Token {
    pub fn eof(line) -> Token{
        Token { ttype: TokenType::EOF, lexeme: "".to_owned(), literal: None, line }
    }
}
