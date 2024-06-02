mod boolean_expression;
mod call_expression;
mod function_literal;
mod identifier;
mod if_expression;
mod infix_expression;
mod integer_literal;
mod prefix_expression;

pub use boolean_expression::*;
pub use call_expression::*;
pub use function_literal::*;
pub use identifier::*;
pub use if_expression::*;
pub use infix_expression::*;
pub use integer_literal::*;
pub use prefix_expression::*;

#[derive(Debug, Clone)]
pub enum Expression {
    BooleanExpression(BooleanExpression),
    CallExpression(CallExpression),
    FunctionLiteral(FunctionLiteral),
    Identifier(Identifier),
    IfExpression(IfExpression),
    InfixExpression(InfixExpression),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Expression::BooleanExpression(v) => v.to_string(),
            Expression::CallExpression(v) => v.to_string(),
            Expression::FunctionLiteral(v) => v.to_string(),
            Expression::Identifier(v) => v.to_string(),
            Expression::IfExpression(v) => v.to_string(),
            Expression::InfixExpression(v) => v.to_string(),
            Expression::IntegerLiteral(v) => v.to_string(),
            Expression::PrefixExpression(v) => v.to_string(),
        };

        f.write_str(&value)
    }
}
