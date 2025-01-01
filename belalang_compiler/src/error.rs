use crate::lexer::tokens::Token;
use crate::ast::Expression;

#[derive(thiserror::Error, Debug)]
pub enum CompileError {
    #[error("unknown infix operator: {0}")]
    UnknownInfixOp(Token),

    #[error("duplicate symbol: {0}")]
    DuplicateSymbol(String),

    #[error("unknown symbol: {0}")]
    UnknownSymbol(String),
}

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("unexpected EOF")]
    UnexpectedEOF,

    #[error(r"unknown escape string")]
    UnknownEscapeString,

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
