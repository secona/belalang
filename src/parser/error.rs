use crate::token::Token;

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("unknown prefix operator: {0}")]
    PrefixOperator(Token),

    #[error("error parsing integer: could not parse {0} as integer")]
    ParsingInteger(String),
}
