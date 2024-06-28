use belalang_core::ast;
use crate::environment::Environment;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Builtin(String),
    Array(Vec<Object>),

    Function {
        params: Vec<ast::Identifier>,
        body: ast::BlockExpression,
        env: Environment,
    },
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Integer(i) => f.write_str(&format!("{}", i)),
            Self::Float(fl) => f.write_str(&format!("{}", fl)),
            Self::Boolean(b) => f.write_str(&format!("{}", b)),
            Self::String(s) => f.write_str(s),
            Self::Array(a) => f.write_str(&format!("{:?}", a)),
            _ => f.write_str(""),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}
