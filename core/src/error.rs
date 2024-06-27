use crate::ast::Expression;
use crate::token::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("unexpected EOF")]
    UnexpectedEOF,

    #[error(r"unknown escape string: \{0}")]
    UnknownEscapeString(String),

    #[error("unknown prefix operator: {0}")]
    UnknownPrefixOperator(Token),

    #[error("unknown token: {0}")]
    UnknownToken(String),

    #[error("invalid lhs: {0}")]
    InvalidLHS(Expression),

    #[error("error parsing integer: could not parse {0} as integer")]
    ParsingInteger(String),

    #[error("error parsing float: could not parse {0} as float")]
    ParsingFloat(String),

    #[error("unclosed string")]
    UnclosedString(),
}
