macro_rules! arithmetic_tokens {
    () => {
        crate::token::Token::Add
            | crate::token::Token::Sub
            | crate::token::Token::Mul
            | crate::token::Token::Div
            | crate::token::Token::Mod
    };
}

pub(super) use arithmetic_tokens;

macro_rules! comparison_tokens {
    () => {
        crate::token::Token::Eq
            | crate::token::Token::Ne
            | crate::token::Token::Gt
            | crate::token::Token::Ge
            | crate::token::Token::Lt
            | crate::token::Token::Le
    };
}

pub(super) use comparison_tokens;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub enum Token {
    #[default]
    EOF,
    Empty,

    Ident(String),
    Int(String),
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
    And, // &&
    Or,  // ||

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
    Backslash, // \
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
            Token::Mul => "*",
            Token::Div => "/",
            Token::Mod => "%",

            Token::Not => "!",
            Token::And => "&&",
            Token::Or => "||",

            Token::Eq => "==",
            Token::Ne => "!=",
            Token::Lt => "<",
            Token::Le => "<=",
            Token::Gt => ">",
            Token::Ge => ">=",

            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",

            Token::Function => "fn",
            Token::While => "while",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::True => "true",
            Token::False => "false",

            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Backslash => r"\",
        })
    }
}
