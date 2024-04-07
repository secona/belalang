#![allow(dead_code)]

use crate::{ast, token};

use super::Precedence;

impl super::Parser {
    pub fn prefix_fn(&mut self) -> Option<Box<dyn ast::Expression>> {
        println!("{:?}", self.curr_token);
        match &self.curr_token.clone() {
            Some(tok) => match tok {
                token::Token::Ident(_) => Some(self.parse_identifier()),
                token::Token::Int(_) => self.parse_integer_literal(),
                token::Token::Bang => self.parse_prefix_expression(),
                token::Token::Minus => self.parse_prefix_expression(),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_identifier(&self) -> Box<dyn ast::Expression> {
        Box::new(ast::Identifier {
            token: self.curr_token.clone().unwrap(),
            value: self.curr_token.clone().unwrap().to_string(),
        })
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        let literal = self.curr_token.clone().unwrap().to_string();

        match literal.parse::<i64>() {
            Ok(lit) => Some(Box::new(ast::IntegerLiteral {
                token: self.curr_token.clone().unwrap(),
                value: lit,
            })),
            Err(_) => {
                self.errors
                    .push(format!("could not parse {} as integer", literal).to_owned());
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        let prev_token = self.curr_token.clone().unwrap();

        self.next_token();

        Some(Box::new(ast::PrefixExpression {
            token: prev_token,
            operator: self.curr_token.clone().unwrap().to_string(),
            right: self.parse_expression(Precedence::Prefix).unwrap(),
        }))
    }
}
