use crate::ast::Expression;
use crate::evaluator::object::Object;
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

    #[error("unclosed string")]
    UnclosedString(),
}

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    UnknownPrefixOperator(Token, Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(Object, Token, Object),

    #[error("unknown variable: {0}")]
    UnknownVariable(String),

    #[error("not a function")]
    NotAFunction,

    #[error("overwriting builtin: {0}")]
    OverwriteBuiltin(String),

    #[error("variable redeclaration: {0}")]
    VariableRedeclaration(String),

    #[error("illegal returning value: {0}")]
    ReturningValue(Object),
}
