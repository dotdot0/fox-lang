#![allow(dead_code)]
use crate::{token::{*, self}, token_type::TokenType};

pub struct Scanner{
    pub source:  String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize, 
    line: usize
}

impl Scanner{
    pub fn new(source: String) -> Scanner{
        Scanner{
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, String>{
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::eof(self.line));

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn scan_token(&mut self){
        let c = self.advance();
        match c{
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.is_match('='){
                    TokenType::Bang_Equal
                }else{
                    TokenType::Bang
                };
                self.add_token(tok)
            },
            '=' => {
                let tok = if self.is_match('='){
                    TokenType::Equal_Equal
                }else{
                    TokenType::Equal
                };
                self.add_token(tok);
            },
            '<' => {
                let tok = if self.is_match('='){
                    TokenType::Less_Equal
                }else{
                    TokenType::Less
                };
                self.add_token(tok)
            }
            '>' => {
                let tok = if self.is_match('='){
                    TokenType::Greater_Equal
                }else{
                    TokenType::Greater
                };
                self.add_token(tok);
            },
            _ => {
                unreachable!("unexpected character")
            }
    }
    }

    fn advance(&mut self) -> char{
        let result = self.source.chars().nth(self.current).unwrap();
        self.current = self.current + 1;
        result
    }

    fn add_token(&mut self,ttype: TokenType) {
        self.add_token_object(ttype, None)
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>){
        let lexeme: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line))
    }

    fn is_match(&mut self,expected: char) -> bool{
        if self.is_at_end(){
            false
        }
        else if self.source.chars().nth(self.current).unwrap() != expected{
            false
        }
        else{
            self.current +=  1;
            true
        }
    }

    
}  
