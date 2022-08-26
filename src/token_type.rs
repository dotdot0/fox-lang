#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub enum TokenType{
    //Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    //One or Two character Tokens
    Bang,
    Bang_Equal,
    Equal,
    Equal_Equal,
    Greater,
    Greater_Equal,
    Less,
    Less_Equal,

    //Literals
    Identifier,
    STRING,
    Number,

    //Keywords
    And,
    Class,
    Else,
    Fun,
    False,
    If,
    Nil,
    Or,
    Print,
    RETRUN,
    Super,
    This,
    True,
    VAR,
    WHILE,
    EOF
}
