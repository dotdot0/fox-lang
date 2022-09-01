#![allow(dead_code, unused_imports)]
use std::collections::HashMap;

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
                Err(e) => e.report(&"Syntax Error".to_string())
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
            '/' => self.add_token(TokenType::Slash),
            '#' => {
                if self.is_match('*'){
                    while self.peek().unwrap() != '*' && self.peek_next().unwrap() != '#'{
                        if self.peek().unwrap() == '\n'{
                            self.line += 1;
                            self.advance();
                        }
                        self.advance();
                    }
                }else{

                }
                while self.peek().unwrap() != '\n' && !self.is_at_end() {//Keep consuming character until next line ch;
                    self.advance();
                }
            },
            '"' => match self.string(){
                Ok(_) => (),
                Err(e) => e.report(&self.source)
            },            '\r' => (),
            | ' ' | '\t' => (),
            '\n' => self.line += 1,
            '0'..='9' => self.number(),
            'q' => {if self.is_match('\n'){
                std::process::exit(65)
            }else {
                self.identifier()
            }
        },
            //'e' => if self.is_match('x') {
              //  std::process::exit(65);
                // while self.peek().unwrap() != 't'{
                //     self.advance();
                // }
                // let code: &str = &self.source[self.start..self.current].trim();
                // if code == "exit"{
                //     std::process::exit(65);
                // }
            //},
            _ => if c.is_alphabetic() && !c.is_whitespace(){
                self.identifier();
            }
            else{
            
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
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line, self.current))
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

    fn peek_next(&self) -> Option<char>{
        if self.is_at_end(){
            Some('\0')
        }
        else{
            self.source.chars().nth(self.current + 1)
        }
    }

    fn string(&mut self) -> Result<(), LoxError>{
        while self.peek().unwrap() != '"' && !self.is_at_end(){
            if self.peek().unwrap() == '\n'{
                self.line += 1;
                self.advance();
            }
            self.advance();
        }

        if self.is_at_end(){
            Err(LoxError::error(self.line, "Unterminated string.".to_owned()))
        }
        else{
            self.advance();
            let value: String = self.source[self.start + 1 .. self.current -1].trim().to_string();
            self.add_token_object(TokenType::STRING, Some(Object::Str(value)));
            Ok(())
        }
    }

    fn number(&mut self){
        while self.peek().unwrap().is_numeric(){
            self.advance();
        }
        if self.peek().unwrap() == '.' && self.peek_next().unwrap().is_numeric(){
            self.advance();//Consuming the .
            while self.peek().unwrap().is_numeric(){
                self.advance();
            }
        }
        let value: f32 = self.source[self.start..self.current].to_string().parse().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(value)))
    }

    fn identifier(&mut self){

        while self.peek().unwrap().is_alphanumeric(){
            self.advance();
        }
        let ident:&str = self.source[self.start..self.current].as_ref();
         let tok = self.reserved_keyword(ident);
         //if reserved_keyword.contains_key(ident){
        //     reserved_keyword.get(&ident).unwrap()
        // }else{
        //     &TokenType::Identifier
        // };
        self.add_token_object(tok.0, tok.1)
    }
    
    //Checks if the provided identifier is reserved keyword
    fn reserved_keyword(&self, ident: &str) -> (TokenType, Option<Object>){
        match ident{
            "class" => (TokenType::Class, None),
            "super" => (TokenType::Super, None),
            "and" => (TokenType::And, None),
            "or" => (TokenType::Or, None),
            "false" => (TokenType::False, Some(Object::Bool(false))),
            "true" => (TokenType::True, Some(Object::Bool(true))),
            "nil" => (TokenType::Nil, Some(Object::Nil)),
            "if" => (TokenType::If, None),
            "else" => (TokenType::Else, None),
            "let" | "var" => (TokenType::VAR, None),
            "this" => (TokenType::This, None),
            "fn" => (TokenType::Fun, None),
            "while" => (TokenType::WHILE, None),
            "print" => (TokenType::Print, None),
            "return" => (TokenType::RETRUN, None),
            _ => (TokenType::Identifier, None)
        }
    }

}  

