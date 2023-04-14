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


impl Object{
    pub fn value(&self) -> String{
        match &self {
            Self::Num(x) => x.to_string(),
            Self::Str(s) => s.to_owned(),
            Self::Bool(t) => t.to_string(),
            Self::Nil => String::from("nil")
        }
    }
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
    pub fn tok_to_str(&self) -> String{
        return format!("ttype: {:?},lexeme: {:?},literal: {:?},line: {:?}", self.ttype, self.lexeme, self.literal, self.line);
    }
}  
