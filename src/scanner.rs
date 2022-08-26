use crate::token::*;

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

    pub fn scan_token() -> Result<>{

    }
}  
