#![allow(dead_code)]
//use std::usize;

use crate::token_type::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object{
    Num(f32),
    Str(String),
    Bool(bool),
    Nil
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token{
     pub ttype: TokenType,
     pub lexeme: String,
     pub literal: Option<Object>,
     pub line: usize,
     pub column: usize
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line:usize, column: usize) -> Token{
        Token {
            ttype,
            lexeme,
            literal,
            line,
            column
        }
    }
    pub fn eof(line:usize) -> Token{
        Token { ttype: TokenType::EOF, lexeme: "".to_owned(), literal: None, line , column: 0}
    }
}  