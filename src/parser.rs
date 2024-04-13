mod infix;
mod prefix;

use crate::{ast, lexer, token};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn from(tok: &token::Token) -> Self {
        match tok {
            token::Token::Eq | token::Token::NotEq => Self::Equals,
            token::Token::LT | token::Token::GT => Self::LessGreater,
            token::Token::Plus | token::Token::Minus => Self::Sum,
            token::Token::Slash | token::Token::Asterisk => Self::Product,
            _ => Self::Lowest,
        }
    }
}

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Parser {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            curr_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        println!("{:?}", &self.curr_token);
        self.curr_token = self.peek_token.clone();

        let token = self.lexer.next_token();
        self.peek_token = token;
    }

    fn curr_token_is(&self, other: token::Token) -> bool {
        self.curr_token == other
    }

    fn peek_token_is(&self, other: token::Token) -> bool {
        self.peek_token == other
    }

    fn curr_precedence(&self) -> Precedence {
        let curr = self.curr_token.clone();
        Precedence::from(&curr)
    }

    fn peek_precedence(&self) -> Precedence {
        let peek = self.peek_token.clone();
        Precedence::from(&peek)
    }

    fn expect_peek(&mut self, other: token::Token) -> bool {
        if self.peek_token_is(other) {
            self.next_token();
            return true;
        }

        false
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        while !self.curr_token_is(token::Token::EOF) {
            if let Some(stmt) = self.parse_statement() {
                program.add_stmt(stmt)
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.curr_token {
            token::Token::Let => self.parse_let_statement(),
            token::Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let let_token = self.curr_token.clone();

        if !self.expect_peek(token::Token::Ident("".into())) {
            return None;
        }

        let name = ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.clone().to_string(),
        };

        if !self.expect_peek(token::Token::Assign) {
            return None;
        }

        // TODO: parse expression
        while !self.curr_token_is(token::Token::Semicolon) {
            self.next_token();
        }

        let stmt = ast::LetStatement {
            name,
            token: let_token,
            value: Box::new(ast::Identifier {
                value: "5".to_owned(), // TODO: Change this
                token: token::Token::Ident("5".to_owned()),
            }),
        };

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let stmt = ast::ReturnStatement {
            token: self.curr_token.clone(),
            return_value: Box::new(ast::Identifier {
                value: "5".to_owned(), // TODO: Change this
                token: token::Token::Ident("5".to_owned()),
            }),
        };

        self.next_token();

        // TODO: parse expression
        while !self.curr_token_is(token::Token::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let stmt = ast::ExpressionStatement {
            token: self.curr_token.clone(),
            expression: self.parse_expression(Precedence::Lowest).unwrap(),
        };

        if self.peek_token_is(token::Token::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn ast::Expression>> {
        let prefix = self.prefix_fn();

        if prefix.is_none() {
            return None;
        }

        let mut left_expr: Result<Box<dyn ast::Expression>, Box<dyn ast::Expression>> =
            Ok(prefix.unwrap());

        'l: while !self.peek_token_is(token::Token::Semicolon)
            && precedence < self.peek_precedence()
        {
            left_expr = self.infix_fn(&self.peek_token.clone(), left_expr.unwrap());

            if let Err(expr) = left_expr {
                left_expr = Ok(expr);
                break 'l;
            }
        }

        Some(left_expr.unwrap())
    }

    fn parse_block_statement(&mut self) -> ast::BlockStatement {
        let token = self.curr_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !self.curr_token_is(token::Token::RBrace) && !self.curr_token_is(token::Token::EOF) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        ast::BlockStatement { statements, token }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::{
        ast::{self, Node},
        lexer, token,
    };

    #[test]
    fn let_statements() {
        let input = "let x = 5;".to_owned().into_bytes().into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let program = parser.parse_program();
        let tok = program.token();

        if let Some(inner) = tok {
            assert_eq!(*inner, token::Token::Let);
        }
    }

    #[test]
    fn integer_literal_expression() {
        let input = "5;".to_owned().into_bytes().into_boxed_slice();

        let lexer = lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let literal = stmt
            .expression
            .downcast_ref::<ast::IntegerLiteral>()
            .expect("not a(n) ast::IntegerLiteral");

        assert_eq!(literal.value, 5);
        assert_eq!(literal.token().unwrap(), &token::Token::Int("5".to_owned()));
    }

    struct PrefixTest {
        input: Box<[u8]>,
        exp_operator: String,
        exp_right: String,
    }

    #[test]
    fn test_prefix_parsing() {
        let tests: [PrefixTest; 4] = [
            PrefixTest {
                input: "!5;".to_owned().into_bytes().into_boxed_slice(),
                exp_operator: String::from("!"),
                exp_right: String::from("5"),
            },
            PrefixTest {
                input: "-15;".to_owned().into_bytes().into_boxed_slice(),
                exp_operator: String::from("-"),
                exp_right: String::from("15"),
            },
            PrefixTest {
                input: "!true;".to_owned().into_bytes().into_boxed_slice(),
                exp_operator: String::from("!"),
                exp_right: String::from("true"),
            },
            PrefixTest {
                input: "!false;".to_owned().into_bytes().into_boxed_slice(),
                exp_operator: String::from("!"),
                exp_right: String::from("false"),
            },
        ];

        for test in tests {
            let lexer = lexer::Lexer::new(test.input);
            let mut parser = super::Parser::new(lexer);

            let program = parser.parse_program();

            let stmt = program.statements[0]
                .downcast_ref::<ast::ExpressionStatement>()
                .expect("not a(n) ast::ExpressionStatement");

            let exp = stmt
                .expression
                .downcast_ref::<ast::PrefixExpression>()
                .expect("not a(n) ast::PrefixExpression");

            assert_eq!(exp.operator, test.exp_operator);
            assert_eq!((*exp.right).to_string(), test.exp_right);
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests: [[&str; 2]; 22] = [
            ["a * b + c", "((a * b) + c)"],
            ["!-a", "(!(-a))"],
            ["a + b + c", "((a + b) + c)"],
            ["a + b - c", "((a + b) - c)"],
            ["a * b * c", "((a * b) * c)"],
            ["a * b / c", "((a * b) / c)"],
            ["a + b / c", "(a + (b / c))"],
            ["a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"],
            ["3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"],
            ["5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"],
            ["5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"],
            [
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ],
            [
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ],
            ["true", "true"],
            ["false", "false"],
            ["3 > 5 == false", "((3 > 5) == false)"],
            ["3 < 5 == true", "((3 < 5) == true)"],
            ["1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"],
            ["(5 + 5) * 2", "((5 + 5) * 2)"],
            ["2 / (5 + 5)", "(2 / (5 + 5))"],
            ["-(5 + 5)", "(-(5 + 5))"],
            ["!(true == true)", "(!(true == true))"],
        ];

        for test in tests {
            let input = test[0].to_owned().into_bytes().into_boxed_slice();
            let lexer = lexer::Lexer::new(input);
            let mut parser = super::Parser::new(lexer);

            let program = parser.parse_program();
            assert_eq!(program.to_string(), test[1]);
        }
    }
}
