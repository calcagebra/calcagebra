use std::collections::HashMap;

use crate::{
    ast::{Ast, Expression},
    token::Token,
};

pub struct Interpreter {
    ast: Vec<Ast>,
    global_vars: HashMap<String, i32>,
    scope_vars: HashMap<String, i32>,
    functions: HashMap<String, (Vec<String>, Expression)>,
}

impl Interpreter {
    pub fn new(ast: Vec<Ast>) -> Self {
        Self {
            ast,
            global_vars: HashMap::new(),
            scope_vars: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let ast = &self.ast;

        for node in ast {
            match node {
                Ast::Assignment(name, expr) => {
                    self.global_vars
                        .insert(name.to_string(), self.parse_expression(expr));
                }
                Ast::Print(exprs) => {
                    for expr in exprs {
											println!("{}", self.parse_expression(expr));
										}
                }
                Ast::FunctionDeclaration(name, args, code) => {
									self.functions.insert(name.to_string(), (args.to_vec(), code.clone()));
								},
            }
        }
    }

    pub fn parse_expression(&self, expr: &Expression) -> i32 {
        match expr {
            Expression::Binary(lhs, op, rhs) => match op {
                Token::Add => self.parse_expression(lhs) + self.parse_expression(rhs),
                Token::Sub => self.parse_expression(lhs) - self.parse_expression(rhs),
                Token::Mul => self.parse_expression(lhs) * self.parse_expression(rhs),
                Token::Div => self.parse_expression(lhs) / self.parse_expression(rhs),
                Token::Pow => self
                    .parse_expression(lhs)
                    .pow(self.parse_expression(rhs).try_into().unwrap()),
                _ => unimplemented!(),
            },
            Expression::Identifier(ident) => {
                if self.global_vars.get(ident).is_some() {
                    *self.global_vars.get(ident).unwrap()
                } else if self.scope_vars.get(ident).is_some() {
                    *self.scope_vars.get(ident).unwrap()
                } else {
                    panic!("attempt to access value of not assigned identifier `{ident}`")
                }
            }
            Expression::Number(n) => *n,
        }
    }
}
