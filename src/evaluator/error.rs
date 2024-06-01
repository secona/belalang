use crate::object;

#[derive(thiserror::Error, Debug)]
pub enum EvaluatorError<'a> {
    #[error("unknown operator: {0}{1}")]
    PrefixOperator(String, object::Object<'a>),

    #[error("unknown operator: {0} {1} {2}")]
    UnknownInfixOperator(object::Object<'a>, String, object::Object<'a>),

    #[error("identifier not found: {0}")]
    IdentifierNotFound(String),
}
