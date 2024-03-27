#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Empty,
    EOF,

    Ident(&'a [u8]),
    Int(&'a [u8]),
    Illegal(&'a [u8]),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,
    Eq,
    NotEq,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token<'_> {
    pub fn lookup_ident(ident: &[u8]) -> Token {
        match ident {
            b"let" => Token::Let,
            b"fn" => Token::Function,
            b"true" => Token::True,
            b"false" => Token::False,
            b"if" => Token::If,
            b"else" => Token::Else,
            b"return" => Token::Return,
            _ => Token::Ident(ident),
        }
    }
}
