use crate::token::*;

pub struct Scanner{
    pub source:  String,
    pub tokens: Vec<Token>
}

impl Scanner{
    pub fn new(source: String) -> Scanner{
        Scanner{
            source,
            tokens: Vec::new()
        }
    }

    pub fn scan_token(){

    }
}  
