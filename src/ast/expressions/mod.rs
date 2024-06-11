mod boolean_expression;
mod call_expression;
mod function_literal;
mod identifier;
mod if_expression;
mod infix_expression;
mod integer_literal;
mod prefix_expression;
mod string_literal;

pub use boolean_expression::*;
pub use call_expression::*;
pub use function_literal::*;
pub use identifier::*;
pub use if_expression::*;
pub use infix_expression::*;
pub use integer_literal::*;
pub use prefix_expression::*;
pub use string_literal::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Boolean(BooleanExpression),
    Call(CallExpression),
    Function(FunctionLiteral),
    Identifier(Identifier),
    If(IfExpression),
    Infix(InfixExpression),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    String(StringLiteral),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Expression::Boolean(v) => v.to_string(),
            Expression::Call(v) => v.to_string(),
            Expression::Function(v) => v.to_string(),
            Expression::Identifier(v) => v.to_string(),
            Expression::If(v) => v.to_string(),
            Expression::Infix(v) => v.to_string(),
            Expression::Integer(v) => v.to_string(),
            Expression::Prefix(v) => v.to_string(),
            Expression::String(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
