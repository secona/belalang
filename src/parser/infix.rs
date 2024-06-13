use crate::{
    ast::{self, Expression},
    expect_peek, token,
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
            // parse_infix: parse infix expression
            token::Token::Plus
            | token::Token::Minus
            | token::Token::Slash
            | token::Token::Asterisk
            | token::Token::Percent
            | token::Token::Eq
            | token::Token::NotEq
            | token::Token::GT
            | token::Token::LT => {
                let token = self.curr_token.clone();
                let operator = self.curr_token.clone();
                let precedence = Precedence::from(&self.curr_token);

                self.next_token();

                let right = self.parse_expression(precedence).unwrap();

                Ok(Expression::Infix(ast::InfixExpression {
                    token,
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }))
            }

            // parse_call: parse call expression
            token::Token::LParen => {
                if let Ok(args) = self.parse_call_args() {
                    Ok(Expression::Call(ast::CallExpression {
                        token: self.curr_token.clone(),
                        function: Box::new(left),
                        args,
                    }))
                } else {
                    Err(left)
                }
            }
            _ => Err(left),
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
