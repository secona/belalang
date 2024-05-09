use super::Expression;
use crate::{ast::BlockStatement, token};

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl std::fmt::Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string()
        );

        if let Some(stmt) = &self.alternative {
            result += &format!("else {}", stmt.to_string());
        }

        f.write_str(&result)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{ast, lexer, parser, test_util, token};
//
//     #[test]
//     fn works_without_else() {
//         let input = "if (x < y) { x }"
//             .to_owned()
//             .into_bytes()
//             .into_boxed_slice();
//
//         let lexer = lexer::Lexer::new(input);
//         let mut parser = parser::Parser::new(lexer);
//
//         let program = parser.parse_program().expect("got parser errors");
//
//         assert_eq!(program.statements.len(), 1);
//
//         let stmt = program.statements[0]
//             .downcast_ref::<ast::ExpressionStatement>()
//             .expect("not a(n) ast::ExpressionStatement");
//
//         let if_expr = stmt
//             .expression
//             .downcast_ref::<ast::IfExpression>()
//             .expect("not a(n) ast::IfExpression");
//
//         assert!(if_expr.alternative.is_none());
//         assert_eq!(if_expr.condition.to_string(), "(x < y)");
//         assert_eq!(if_expr.token, token::Token::If);
//
//         assert_eq!(
//             if_expr.consequence.statements[0]
//                 .downcast_ref::<ast::ExpressionStatement>()
//                 .expect("not a(n) ast::ExpressionStatement")
//                 .to_string(),
//             "x",
//         );
//     }
//
//     #[test]
//     fn works_with_else() {
//         let input = "if (x < y) { x } else { y }"
//             .to_owned()
//             .into_bytes()
//             .into_boxed_slice();
//
//         let lexer = lexer::Lexer::new(input);
//         let mut parser = parser::Parser::new(lexer);
//
//         let program = parser.parse_program().expect("got parser errors");
//
//         assert_eq!(program.statements.len(), 1);
//
//         let stmt = program.statements[0]
//             .downcast_ref::<ast::ExpressionStatement>()
//             .expect("not a(n) ast::ExpressionStatement");
//
//         let if_expr = stmt
//             .expression
//             .downcast_ref::<ast::IfExpression>()
//             .expect("not a(n) ast::IfExpression");
//
//         assert_eq!(if_expr.token, token::Token::If);
//
//         assert_eq!(if_expr.condition.to_string(), "(x < y)");
//
//         assert_eq!(
//             if_expr.consequence.statements[0]
//                 .downcast_ref::<ast::ExpressionStatement>()
//                 .expect("not a(n) ast::ExpressionStatement")
//                 .to_string(),
//             "x",
//         );
//
//         let alt = if_expr.alternative.as_ref().expect("alternative is None");
//
//         assert_eq!(alt.token, token::Token::LBrace);
//         assert_eq!(
//             alt.statements[0]
//                 .downcast_ref::<ast::ExpressionStatement>()
//                 .expect("not a(n) ast::ExpressionStatement")
//                 .to_string(),
//             "y"
//         );
//     }
//
//     #[test]
//     fn works_multiple_statements() {
//         let input = "if (x < y) { let a = 10; x }"
//             .to_owned()
//             .into_bytes()
//             .into_boxed_slice();
//
//         let lexer = lexer::Lexer::new(input);
//         let mut parser = parser::Parser::new(lexer);
//
//         let program = parser.parse_program().expect("got parser errors");
//
//         assert_eq!(program.statements.len(), 1);
//
//         let stmt = program.statements[0]
//             .downcast_ref::<ast::ExpressionStatement>()
//             .expect("not a(n) ast::ExpressionStatement");
//
//         let if_expr = stmt
//             .expression
//             .downcast_ref::<ast::IfExpression>()
//             .expect("not a(n) ast::IfExpression");
//
//         test_util::test_infix_expression(
//             if_expr.condition.as_ref(),
//             test_util::Expected::Ident("x"),
//             "<",
//             test_util::Expected::Ident("y"),
//         );
//
//         assert!(if_expr.alternative.is_none());
//         assert_eq!(if_expr.token, token::Token::If);
//
//         assert_eq!(
//             if_expr.consequence.statements[0]
//                 .downcast_ref::<ast::LetStatement>()
//                 .expect("not a(n) ast::LetStatement")
//                 .to_string(),
//             "let a = 10;",
//         );
//
//         assert_eq!(
//             if_expr.consequence.statements[1]
//                 .downcast_ref::<ast::ExpressionStatement>()
//                 .expect("not a(n) ast::ExpressionStatement")
//                 .to_string(),
//             "x",
//         );
//     }
// }
