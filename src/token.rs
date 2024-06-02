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

impl ToString for Token {
    fn to_string(&self) -> String {
        String::from(match self {
            Token::Empty => "",
            Token::EOF => "",
            Token::Ident(s) => s,
            Token::Int(s) => s,
            Token::Illegal(s) => s,
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Bang => "!",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::LT => "<",
            Token::GT => ">",
            Token::Eq => "==",
            Token::NotEq => "!=",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Function => "fn",
            Token::Let => "let",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
        })
    }
}
