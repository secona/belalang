use crate::object;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError {
    #[error("unknown operator: {0}{1}")]
    PrefixOperator(String, object::Object),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(object::Object, String, object::Object),

    #[error("identifier not found: {0}")]
    IdentifierNotFound(String),

    #[error("not a function")]
    NotAFunction(),
}
