#![allow(dead_code)]

use crate::{ast, token};

impl super::Parser {
    pub fn infix_fn(
        &mut self,
        tok: &token::Token,
        left: Box<dyn ast::Expression>,
    ) -> Result<Box<dyn ast::Expression>, Box<dyn ast::Expression>> {
        println!("{:?}", tok);
        match tok {
            token::Token::Plus
            | token::Token::Minus
            | token::Token::Slash
            | token::Token::Asterisk
            | token::Token::Eq
            | token::Token::NotEq
            | token::Token::GT
            | token::Token::LT => self.parse_infix_expression(left),
            _ => Err(left),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left: Box<dyn ast::Expression>,
    ) -> Result<Box<dyn ast::Expression>, Box<dyn ast::Expression>> {
        self.next_token();

        let token = self.curr_token.clone().unwrap();
        let operator = self.curr_token.clone().unwrap().to_string();
        let precedence = self.curr_precedence();

        self.next_token();

        Ok(Box::new(ast::InfixExpression {
            token,
            left,
            operator,
            right: self.parse_expression(precedence).unwrap(),
        }))
    }
}
