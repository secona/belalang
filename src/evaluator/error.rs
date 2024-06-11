use super::object::Object;
use crate::token::Token;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    PrefixOperator(Token, Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(Object, Token, Object),

    #[error("unknown variable: {0}")]
    UnknownVariable(String),

    #[error("not a function")]
    NotAFunction(),

    #[error("overwriting builtin: {0}")]
    OverwriteBuiltin(String),

    #[error("variable redeclaration: {0}")]
    VariableRedeclaration(String),

    #[error("illegal returning value: {0}")]
    ReturningValue(Object),

    #[error("unexpected token: {0}")]
    UnexpectedToken(Token)
}
