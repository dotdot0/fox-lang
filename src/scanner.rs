#![allow(dead_code, unused_imports)]
use crate::{token::{*, self}, token_type::TokenType};
use crate::error::LoxError;

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
            //println!("{}", self.current);
            match self.scan_token(){
                Ok(_) => (),
                Err(e) => e.report(&self.source)
            }  
        }
        self.tokens.push(Token::eof(self.line));

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError>{
        let c = self.advance();
        //println!("{}", self.advance());
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
            '/' => {
                if self.is_match('/'){
                    while self.peek().unwrap() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }else{
                    self.add_token(TokenType::Slash)
                }
            },
            // '"' => match self.string(){
            //     Ok(_) => (),
            //     Err(e) => e.report(&self.source)
            // },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            'q' => std::process::exit(64),
            _ => {
                return Err(LoxError::error(self.line, "Unexpected Character".to_owned()));
            }


    }
    Ok(())
    }

    fn advance(&mut self) -> char{
        let result = self.source.chars().nth(self.current).unwrap();
        self.current = self.current + 1;
        //println!("{}", result);
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

    fn peek(&mut self) -> Option<char>{
        if self.is_at_end() { 
            Some('\0')
        }
        else{
            self.source.chars().nth(self.current)
        }
    }

    fn string(&mut self) -> Result<(), LoxError>{
        while self.peek().unwrap() != '"' && !self.is_at_end(){
            if self.peek().unwrap() == '\n'{
                self.line += 1;
                self.advance();
            }
        }

        if self.is_at_end(){
            Err(LoxError::error(self.line, "Unterminated string.".to_owned()))
        }
        else{
            self.advance();
            let value: String = self.source[self.start + 1 .. self.current -1].to_string();
            self.add_token_object(TokenType::STRING, Some(Object::Str(value)));
            Ok(())
        }
    }
    
}  
