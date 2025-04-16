mod lexer;

pub use lexer::Lexer;

macro_rules! arithmetic_tokens {
    () => {
        Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Mod
    };
}

pub(crate) use arithmetic_tokens;

macro_rules! comparison_tokens {
    () => {
        Token::Eq | Token::Ne | Token::Gt | Token::Ge | Token::Lt | Token::Le
    };
}

pub(crate) use comparison_tokens;

macro_rules! assignment_tokens {
    () => {
        Token::Assign
            | Token::ColonAssign
            | Token::AddAssign
            | Token::SubAssign
            | Token::MulAssign
            | Token::DivAssign
            | Token::ModAssign
            | Token::BitAndAssign
            | Token::BitOrAssign
            | Token::BitXorAssign
            | Token::ShiftLeftAssign
            | Token::ShiftRightAssign
    };
}

pub(crate) use assignment_tokens;

macro_rules! bitwise_tokens {
    () => {
        Token::BitAnd | Token::BitOr | Token::BitXor | Token::ShiftLeft | Token::ShiftRight
    };
}

pub(crate) use bitwise_tokens;

use crate::error::SyntaxError;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub enum Token {
    #[default]
    EOF,
    Empty,

    Ident(String),
    Int(String),
    Float(String),
    String(String),

    // Assignment operators
    Assign,           // =
    ColonAssign,      // :=
    AddAssign,        // +=
    SubAssign,        // -=
    MulAssign,        // *=
    DivAssign,        // /=
    ModAssign,        // %=
    BitAndAssign,     // &=
    BitOrAssign,      // |=
    BitXorAssign,     // ^=
    ShiftLeftAssign,  // <<=
    ShiftRightAssign, // >>=

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

    // Bitwise operators
    BitAnd, // &
    BitOr,  // |
    // BitNot,  // ~ TODO
    BitXor,     // ^
    ShiftLeft,  // <<
    ShiftRight, // >>

    // Comparison operators
    Eq, // ==
    Ne, // !=
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=

    // Parenthesis and Braces
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

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

impl TryFrom<&[u8]> for Token {
    type Error = SyntaxError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"fn" => Ok(Token::Function),
            b"while" => Ok(Token::While),
            b"true" => Ok(Token::True),
            b"false" => Ok(Token::False),
            b"if" => Ok(Token::If),
            b"else" => Ok(Token::Else),
            b"return" => Ok(Token::Return),
            _ => String::from_utf8(value.to_vec())
                .map(Token::Ident)
                .map_err(|_| SyntaxError::InvalidUtf8Character),
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
            Token::Float(s) => s,
            Token::String(s) => s,

            Token::Assign => "=",
            Token::ColonAssign => ":=",
            Token::AddAssign => "+=",
            Token::SubAssign => "-=",
            Token::MulAssign => "*=",
            Token::DivAssign => "/=",
            Token::ModAssign => "%=",
            Token::BitAndAssign => "&=",
            Token::BitOrAssign => "|=",
            Token::BitXorAssign => "^=",
            Token::ShiftLeftAssign => "<<=",
            Token::ShiftRightAssign => ">>=",

            Token::Add => "+",
            Token::Sub => "-",
            Token::Mul => "*",
            Token::Div => "/",
            Token::Mod => "%",

            Token::Not => "!",
            Token::And => "&&",
            Token::Or => "||",

            Token::BitAnd => "&",
            Token::BitOr => "|",
            // Token::BitNot => "~", TODO
            Token::BitXor => "^",
            Token::ShiftLeft => "<<",
            Token::ShiftRight => ">>",

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
            Token::LeftBracket => "[",
            Token::RightBracket => "]",

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
