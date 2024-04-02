use std::{
    collections::HashMap, f32::consts::{PI, E}, io::{stdin, stdout, Write}
};

use crate::{
    ast::{Ast, Expression},
    token::Token,
};

#[derive(Debug)]
pub struct Interpreter {
    global_vars: HashMap<String, f32>,
    scope_vars: HashMap<String, f32>,
    functions: HashMap<String, (Vec<String>, Expression)>,
    std: HashMap<String, fn(Vec<f32>) -> f32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            global_vars: HashMap::new(),
            scope_vars: HashMap::new(),
            functions: HashMap::new(),
            std: HashMap::new(),
        }
    }

    pub fn init_std(&mut self) {
        self.std.insert("print".to_string(), |x| {
            x.iter().for_each(|a| println!("{a}"));
            0.0
        });
        self.std.insert("read".to_string(), |_| {
            print!("Enter number: ");
            stdout().flush().unwrap();
            let mut buf = String::new();

            stdin().read_line(&mut buf).unwrap();

            buf.trim_end().parse::<f32>().unwrap()
        });

        self.std.insert("log".to_string(), |x| x[0].ln());
    }

    pub fn init_globals(&mut self) {
        [("pi", PI), ("e", E)].map(|(k,v)|self.global_vars.insert(k.to_string(), v));
    }

    pub fn run(&mut self, ast: Vec<Ast>) {
        self.init_std();
        self.init_globals();
        for node in ast {
            match node {
                Ast::Assignment(name, expr) => {
                    let r = self.parse_expression(&expr);
                    self.global_vars.insert(name.to_string(), r);
                }
                Ast::FunctionCall(i, exprs) => {
                    if self.std.get(&i).is_some() {
                        let f = self.std.get(&i).unwrap();
                        f(exprs.iter().map(|x| self.parse_expression(x)).collect());
                    } else {
                        let (args, code) = self.functions.get(&i).unwrap().clone();

                        let scope_vars = self.scope_vars.clone();

                        for (i, arg) in args.iter().enumerate() {
                            let r = self.parse_expression(&exprs[i]);
                            self.scope_vars.insert(arg.to_string(), r);
                        }

                        self.parse_expression(&code);

                        self.scope_vars = scope_vars;
                    }
                }
                Ast::FunctionDeclaration(name, args, code) => {
                    self.functions
                        .insert(name.to_string(), (args.to_vec(), code.clone()));
                }
            }
        }
    }

    pub fn parse_expression(&mut self, expr: &Expression) -> f32 {
        match expr {
            Expression::Binary(lhs, op, rhs) => match op {
                Token::Add => self.parse_expression(lhs) + self.parse_expression(rhs),
                Token::Sub => self.parse_expression(lhs) - self.parse_expression(rhs),
                Token::Mul => self.parse_expression(lhs) * self.parse_expression(rhs),
                Token::Div => self.parse_expression(lhs) / self.parse_expression(rhs),
                Token::Pow => self
                    .parse_expression(lhs)
                    .powf(self.parse_expression(rhs)),
                _ => unimplemented!(),
            },
            Expression::Identifier(ident) => {
                if self.global_vars.get(ident).is_some() {
                    *self.global_vars.get(ident).unwrap()
                } else if self.scope_vars.get(ident).is_some() {
                    *self.scope_vars.get(ident).unwrap()
                } else if self.std.get(ident).is_some() {
                    let f = self.std.get(ident).unwrap();
                    f(vec![])
                } else {
                    panic!("attempt to access value of not assigned identifier `{ident}`")
                }
            }
            Expression::Number(n) => *n,
            Expression::FunctionCall(i, exprs) => {
                if self.std.get(i).is_some() {
                    let f = self.std.get(i).unwrap();
                    return f(exprs.iter().map(|x| self.parse_expression(x)).collect());
                }
                let (args, code) = self.functions.get(i).unwrap().clone();

                let scope_vars = self.scope_vars.clone();

                for (i, arg) in args.iter().enumerate() {
                    let r = self.parse_expression(&exprs[i]);
                    self.scope_vars.insert(arg.to_string(), r);
                }

                let r = self.parse_expression(&code);

                self.scope_vars = scope_vars;

                r
            }
        }
    }
}
