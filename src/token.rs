#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Empty,
    EOF,

    Ident(String),
    Int(String),
    Illegal(String),

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

impl Token {
    pub fn lookup_ident(ident: &[u8]) -> Token {
        match ident {
            b"let" => Token::Let,
            b"fn" => Token::Function,
            b"true" => Token::True,
            b"false" => Token::False,
            b"if" => Token::If,
            b"else" => Token::Else,
            b"return" => Token::Return,
            _ => Token::Ident(String::from_utf8(ident.to_vec()).unwrap()),
        }
    }
}
