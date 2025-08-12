mod lexer;

pub use lexer::*;

#[macro_export]
macro_rules! arithmetic_tokens {
    () => {
        Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Mod
    };
}

#[macro_export]
macro_rules! comparison_tokens {
    () => {
        Token::Eq | Token::Ne | Token::Gt | Token::Ge | Token::Lt | Token::Le
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! bitwise_tokens {
    () => {
        Token::BitAnd | Token::BitOr | Token::BitXor | Token::ShiftLeft | Token::ShiftRight
    };
}

/// Belalang language's tokens
///
/// This is all tokens that exist in the belalang language grammar.
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub enum Token {
    /// End of file marker
    #[default]
    EOF,

    /// Empty token placeholder
    Empty,

    /// Identifier token containing a variable or function name
    Ident(String),
    /// Integer literal
    Int(String),
    /// Floating point literal
    Float(String),
    /// String literal
    String(String),

    /// Assignment operator `=`
    Assign,
    /// Colon assignment operator `:=`
    ColonAssign,
    /// Addition assignment operator `+=`
    AddAssign,
    /// Subtraction assignment operator `-=`
    SubAssign,
    /// Multiplication assignment operator `*=`
    MulAssign,
    /// Division assignment operator `/=`
    DivAssign,
    /// Modulo assignment operator `%=`
    ModAssign,
    /// Bitwise AND assignment operator `&=`
    BitAndAssign,
    /// Bitwise OR assignment operator `|=`
    BitOrAssign,
    /// Bitwise XOR assignment operator `^=`
    BitXorAssign,
    /// Shift left assignment operator `<<=`
    ShiftLeftAssign,
    /// Shift right assignment operator `>>=`
    ShiftRightAssign,

    /// Addition operator `+`
    Add,
    /// Subtraction operator `-`
    Sub,
    /// Multiplication operator `*`
    Mul,
    /// Division operator `/`
    Div,
    /// Modulo operator `%`
    Mod,

    /// Logical NOT operator `!`
    Not,
    /// Logical AND operator `&&`
    And,
    /// Logical OR operator `||`
    Or,

    /// Bitwise AND operator `&`
    BitAnd,
    /// Bitwise OR operator `|`
    BitOr,
    /// Bitwise XOR operator `^`
    BitXor,
    /// Shift left operator `<<`
    ShiftLeft,
    /// Shift right operator `>>`
    ShiftRight,

    /// Equality comparison operator `==`
    Eq,
    /// Inequality comparison operator `!=`
    Ne,

    /// Less than operator `<`
    Lt,
    /// Less than or equal operator `<=`
    Le,
    /// Greater than operator `>`
    Gt,
    /// Greater than or equal operator `>=`
    Ge,

    /// Left parenthesis `()`
    LeftParen,
    /// Right parenthesis `)`
    RightParen,

    /// Left brace `{`
    LeftBrace,
    /// Right brace `}`
    RightBrace,

    /// Left bracket `[`
    LeftBracket,
    /// Right bracket `]`
    RightBracket,

    /// Function keyword `fn`
    Function,
    /// While loop keyword `while`
    While,
    /// If conditional keyword `if`
    If,
    /// Else conditional keyword `else`
    Else,
    /// Return keyword `return`
    Return,
    /// Boolean true literal `true`
    True,
    /// Boolean false literal `false`
    False,

    /// Comma separator `,`
    Comma,
    /// Semicolon terminator `;`
    Semicolon,
    /// Backslash character `\`
    Backslash,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "fn" => Token::Function,
            "while" => Token::While,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(value.to_string()),
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
