#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Empty,
    EOF,

    Ident(String),
    Int(String),
    Illegal(String),
    String(String),

    Walrus,
    Assign,

    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Percent,

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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Token::Empty => "",
            Token::EOF => "",
            Token::Ident(s) => s,
            Token::Int(s) => s,
            Token::Illegal(s) => s,
            Token::String(s) => s,
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
            Token::Walrus => ":=",
            Token::Percent => "%",
        })
    }
}
