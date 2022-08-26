use crate::{token::*, token_type::TokenType};

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

    pub fn scan_tokens(&mut self) -> Result<Vec<&Token>, String>{
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::eof(self.line));

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool{
        self.current > self.source.len()
    }

    fn scan_token(&self){
        let c = self.advance();

        match c{

        }
    }

    fn advance(&self) -> char{
        self.source.chars().nth(self.current + 1).unwrap();
    }
}  
