#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub enum Token {
    #[default]
    EOF,
    Empty,

    Ident(String),
    Int(String),
    Illegal(String),
    String(String),

    // Assignment operators
    Assign,      // =
    ColonAssign, // :=
    AddAssign,   // +=
    SubAssign,   // -=
    MulAssign,   // *=
    DivAssign,   // /=
    ModAssign,   // %=

    // Arithmetic operators
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    // Logical operators
    Not, // !

    // Comparison operators
    Eq, // ==
    Ne, // !=
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=

    // Parenthesis and Braces
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }

    // Keywords
    Function, // fn
    While,    // while
    If,       // if
    Else,     // else
    Return,   // return
    True,     // true
    False,    // false

    // Other tokens
    Comma,     // ,
    Semicolon, // ;
}

impl From<&[u8]> for Token {
    fn from(value: &[u8]) -> Self {
        match value {
            b"fn" => Token::Function,
            b"while" => Token::While,
            b"true" => Token::True,
            b"false" => Token::False,
            b"if" => Token::If,
            b"else" => Token::Else,
            b"return" => Token::Return,
            _ => Token::Ident(String::from_utf8(value.to_vec()).unwrap()),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Token::Empty => "<empty>",
            Token::EOF => "EOF",
            Token::Ident(s) => s,
            Token::Int(s) => s,
            Token::Illegal(s) => s,
            Token::String(s) => s,
            Token::Assign => "=",
            Token::ColonAssign => ":=",
            Token::AddAssign => "+=",
            Token::SubAssign => "-=",
            Token::MulAssign => "*=",
            Token::DivAssign => "/=",
            Token::ModAssign => "%=",
            Token::Add => "+",
            Token::Sub => "-",
            Token::Not => "!",
            Token::Mul => "*",
            Token::Div => "/",
            Token::Lt => "<",
            Token::Le => "<=",
            Token::Gt => ">",
            Token::Ge => ">=",
            Token::Eq => "==",
            Token::Ne => "!=",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::Function => "fn",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::Mod => "%",
            Token::While => "while",
        })
    }
}
