use super::Expression;
use crate::{ast::BlockStatement, token};

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl std::fmt::Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if ({}) {} else {}",
            self.condition,
            self.consequence,
            match &self.alternative {
                Some(alt) => alt.to_string(),
                None => "{}".into(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, testing, token};

    #[test]
    fn parsing_without_else() {
        let input = b"if (x < y) { x }";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let if_expr = testing::as_variant!(&stmt.expression, ast::Expression::IfExpression);

        assert_eq!(if_expr.token, token::Token::If);

        // testing the condition
        testing::expr_variant!(
            &*if_expr.condition, Infix => (
                ast::Expression::Identifier = "x",
                token::Token::LT,
                ast::Expression::Identifier = "y"
            )
        );

        // testing the consequence block
        let stmt_1 = testing::as_variant!(
            &if_expr.consequence.statements[0],
            ast::Statement::ExpressionStatement
        );
        testing::expr_variant!(&stmt_1.expression, ast::Expression::Identifier = "x");

        // testing the alternative block
        assert!(if_expr.alternative.is_none());
    }

    #[test]
    fn with_else() {
        let input = b"if (x < y) { x } else { y }";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().expect("got parser errors");
        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let if_expr = testing::as_variant!(&stmt.expression, ast::Expression::IfExpression);

        assert_eq!(if_expr.token, token::Token::If);

        // testing the condition
        testing::expr_variant!(
            &*if_expr.condition, Infix => (
                ast::Expression::Identifier = "x",
                token::Token::LT,
                ast::Expression::Identifier = "y"
            )
        );

        // testing the consequence block
        let stmt_0 = testing::as_variant!(
            &if_expr.consequence.statements[0],
            ast::Statement::ExpressionStatement
        );
        testing::expr_variant!(&stmt_0.expression, ast::Expression::Identifier = "x");

        // testing the alternative block
        let alt = if_expr.alternative.as_ref().expect("alternative is None");
        assert_eq!(alt.token, token::Token::LBrace);

        let stmt_0 = testing::as_variant!(&alt.statements[0], ast::Statement::ExpressionStatement);
        testing::expr_variant!(&stmt_0.expression, ast::Expression::Identifier = "y");
    }

    #[test]
    fn parsing_with_multiple_statements() {
        let input = b"if (x < y) { a := 10; x }";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let if_expr = testing::as_variant!(&stmt.expression, ast::Expression::IfExpression);

        testing::expr_variant!(
            if_expr.condition.as_ref(), Infix => (
                ast::Expression::Identifier = "x",
                token::Token::LT,
                ast::Expression::Identifier = "y"
            )
        );

        assert!(if_expr.alternative.is_none());
        assert_eq!(if_expr.token, token::Token::If);

        // testing consequence block
        let stmt_0 = testing::as_variant!(
            &if_expr.consequence.statements[0],
            ast::Statement::Var
        );
        testing::ident_has_name!(stmt_0.name, "a");
        testing::expr_variant!(&stmt_0.value, ast::Expression::IntegerLiteral = 10);

        let stmt_1 = testing::as_variant!(
            &if_expr.consequence.statements[1],
            ast::Statement::ExpressionStatement
        );
        testing::expr_variant!(&stmt_1.expression, ast::Expression::Identifier = "x");
    }
}
