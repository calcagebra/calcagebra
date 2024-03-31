use std::{iter::Peekable, slice::Iter};

use crate::{
    ast::{Ast, Expression},
    token::Token,
};

pub struct Parser {
    tokens: Vec<Vec<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Vec<Token>>) -> Self {
        Self { tokens }
    }

    pub fn ast(&self) -> Vec<Ast> {
        let mut ast = vec![];
        let lines = &self.tokens;

        for line in lines {
            let mut tokens = line.iter().peekable();

            let identifier = tokens.next().unwrap();

            let is_eq = **tokens.peek().unwrap() == Token::Eq;

            if is_eq {
                tokens.next();
                let mut name = "";
                if let Token::Identifier(str) = identifier {
                    name = str;
                }

                let expr = self.pratt_parser(tokens, 0).0;

                ast.push(Ast::Assignment(name.to_string(), expr));
            } else {
                let name = match identifier {
                    Token::Identifier(name) => name,
                    _ => unreachable!(),
                };

                if line.contains(&Token::Eq) {
                    let mut args = vec![];

                    loop {
                        let t = tokens.peek();

                        if t.is_none() || **t.unwrap() == Token::Eq {
                            break;
                        }

                        let t = tokens.next().unwrap();

                        args.push(match t {
                            Token::Identifier(i) => i.to_string(),
                            _ => unreachable!(),
                        })
                    }
                    tokens.next();
                    let expr = self.pratt_parser(tokens, 0).0;
                    ast.push(Ast::FunctionDeclaration(name.to_string(), args, expr));
                } else {
                    let mut args = vec![];

                    loop {
                        let t = tokens.peek();

                        if t.is_none() || **t.unwrap() == Token::Eq {
                            break;
                        }

                        let t = tokens.next().unwrap();

                        args.push(match t {
                            Token::Identifier(i) => Expression::Identifier(i.to_string()),
                            Token::Number(n) => Expression::Number(*n),
                            _ => unreachable!(),
                        })
                    }
                    ast.push(Ast::Print(args));
                }
            }
        }

        ast
    }

    pub fn pratt_parser<'a>(
        &'a self,
        mut tokens: Peekable<Iter<'a, Token>>,
        prec: u16,
    ) -> (Expression, Peekable<Iter<Token>>) {
        let token = tokens.next().unwrap();
        let mut expr: Option<Expression> = None;

        match token {
            Token::Identifier(i) => {
                expr = Some(Expression::Identifier(i.to_string()));
            }
            Token::LParen => {
                let exp;
                (exp, tokens) = self.pratt_parser(tokens, 0);
                expr = Some(exp);
            }
            Token::Sub => {
                if let Token::Number(i) = tokens.peek().unwrap() {
                    expr = Some(Expression::Number(-i));
                    tokens.next();
                }
            }
            _ => {
                if let Token::Number(i) = token {
                    expr = Some(Expression::Number(*i));
                }
            }
        };

        loop {
            let op = tokens.peek();

            if op.is_none() || **op.unwrap() == Token::RParen {
                tokens.next();
                break;
            }

            if **op.unwrap() == Token::Pow && self.infix_binding_power(op.unwrap()) < prec {
                break;
            }

            if **op.unwrap() != Token::Pow && self.infix_binding_power(op.unwrap()) <= prec {
                break;
            }
            let op = tokens.next().unwrap();
            let rhs;
            (rhs, tokens) = self.pratt_parser(tokens, self.infix_binding_power(op));
            expr = Some(Expression::Binary(
                Box::new(expr.unwrap()),
                op.clone(),
                Box::new(rhs),
            ))
        }

        (expr.unwrap(), tokens)
    }

    fn infix_binding_power(&self, op: &Token) -> u16 {
        match op {
            Token::Add => 1,
            Token::Sub => 2,
            Token::Mul => 3,
            Token::Div => 4,
            Token::Pow => 5,
            Token::Neq | Token::Geq | Token::Leq | Token::GT | Token::LT => 6,
            _ => panic!("unexpected operator {op:?}"),
        }
    }
}
