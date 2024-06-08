#![allow(dead_code)]

use crate::{
    ast::{self, Expression}, expect_peek, token
};

use super::{error::ParserError, Precedence};

impl super::Parser<'_> {
    pub fn infix_fn(
        &mut self,
        tok: &token::Token,
        left: Expression,
    ) -> Result<Expression, Expression> {
        self.next_token();

        match tok {
            token::Token::Plus
            | token::Token::Minus
            | token::Token::Slash
            | token::Token::Asterisk
            | token::Token::Percent
            | token::Token::Eq
            | token::Token::NotEq
            | token::Token::GT
            | token::Token::LT => self.parse_infix_expression(left),
            token::Token::LParen => self.parse_call_expression(left),
            _ => Err(left),
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, Expression> {
        let token = self.curr_token.clone();
        let operator = self.curr_token.clone();
        let precedence = self.curr_precedence();

        self.next_token();

        let right = self.parse_expression(precedence).unwrap();

        Ok(Expression::InfixExpression(ast::InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, Expression> {
        if let Ok(args) = self.parse_call_args() {
            Ok(Expression::CallExpression(ast::CallExpression {
                token: self.curr_token.clone(),
                function: Box::new(function),
                args,
            }))
        } else {
            Err(function)
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut args = Vec::new();

        if matches!(self.peek_token, token::Token::RParen) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);

        while matches!(self.peek_token, token::Token::Comma) {
            self.next_token();
            self.next_token();

            args.push(self.parse_expression(Precedence::Lowest)?);
        }

        expect_peek!(self, token::Token::RParen);

        Ok(args)
    }
}
