use crate::{
    ast::{BlockStatement, Expression, Node},
    token,
};

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl ToString for IfExpression {
    fn to_string(&self) -> String {
        let mut result = format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if let Some(stmt) = &self.alternative {
            result += &format!("else {}", stmt.to_string());
        }

        result
    }
}

impl Node for IfExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::{ast, lexer, parser, token};

    #[test]
    fn works_without_else() {
        let input = "if (x < y) { x }"
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        if let Some(stmt) = program.statements[0].downcast_ref::<ast::ExpressionStatement>() {
            if let Some(if_expr) = stmt.expression.downcast_ref::<ast::IfExpression>() {
                assert!(if_expr.alternative.is_none());
                assert_eq!(if_expr.condition.to_string(), "(x < y)");
                assert_eq!(if_expr.token, token::Token::If);
                assert_eq!(if_expr.consequence.to_string(), "x");

                assert_eq!(
                    if_expr.consequence.statements[0]
                        .downcast_ref::<ast::ExpressionStatement>()
                        .unwrap()
                        .to_string(),
                    "x",
                );
            } else {
                panic!("expression not ast::IfExpression");
            }
        } else {
            panic!("statement not ast::ExpressionStatement");
        }
    }

    #[test]
    fn works_multiple_statements() {
        let input = "if (x < y) { let a = 10; x }"
            .to_owned()
            .into_bytes()
            .into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        if let Some(stmt) = program.statements[0].downcast_ref::<ast::ExpressionStatement>() {
            if let Some(if_expr) = stmt.expression.downcast_ref::<ast::IfExpression>() {
                assert!(if_expr.alternative.is_none());
                assert_eq!(if_expr.condition.to_string(), "(x < y)");
                assert_eq!(if_expr.token, token::Token::If);
                assert_eq!(if_expr.consequence.to_string(), "x");

                assert_eq!(
                    if_expr.consequence.statements[0]
                        .downcast_ref::<ast::LetStatement>()
                        .unwrap()
                        .to_string(),
                    "let a = 10",
                );

                assert_eq!(
                    if_expr.consequence.statements[1]
                        .downcast_ref::<ast::ExpressionStatement>()
                        .unwrap()
                        .to_string(),
                    "x",
                );
            } else {
                panic!("expression not ast::IfExpression");
            }
        } else {
            panic!("statement not ast::ExpressionStatement");
        }
    }
}
