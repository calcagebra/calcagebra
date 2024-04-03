use std::{
    collections::HashMap,
    f32::consts::{E, PI},
    io::{stdin, stdout, Write},
};

use crate::{
    ast::{Ast, Expression},
    token::Token,
};

use textplots::{AxisBuilder, Chart, LineStyle, Plot, Shape, TickDisplay, TickDisplayBuilder};

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

        self.std.insert("sin".to_string(), |x| x[0].sin());
        self.std.insert("cos".to_string(), |x| x[0].cos());
        self.std.insert("tan".to_string(), |x| x[0].tan());

        self.std.insert("sqrt".to_string(), |x| x[0].sqrt());
        self.std.insert("cbrt".to_string(), |x| x[0].cbrt());
        self.std
            .insert("nrt".to_string(), |x| x[0].powf(1.0 / x[1]));
    }

    pub fn init_globals(&mut self) {
        [("pi", PI), ("e", E)].map(|(k, v)| self.global_vars.insert(k.to_string(), v));
    }

    pub fn run(&mut self, ast: Vec<Ast>) {
        self.init_std();
        self.init_globals();
        for node in ast {
            match node {
                Ast::Assignment(name, expr) => {
                    let r = self.eval_expression(&expr);
                    self.global_vars.insert(name.to_string(), r);
                }
                Ast::FunctionCall(i, exprs) => {
                    if self.std.get(&i).is_some() {
                        let f = self.std.get(&i).unwrap();
                        f(exprs.iter().map(|x| self.eval_expression(x)).collect());
                    } else {
                        let (args, code) = self.functions.get(&i).unwrap().clone();

                        let scope_vars = self.scope_vars.clone();

                        for (i, arg) in args.iter().enumerate() {
                            let r = self.eval_expression(&exprs[i]);
                            self.scope_vars.insert(arg.to_string(), r);
                        }

                        self.eval_expression(&code);

                        self.scope_vars = scope_vars;
                    }
                }
                Ast::FunctionDeclaration(name, args, code) => {
                    self.functions
                        .insert(name.to_string(), (args.to_vec(), code.clone()));
                }
            }
        }
        self.graph();
    }

    pub fn graph(&mut self) {
        self.functions
            .iter()
            .filter(|f| f.0.starts_with('$'))
            .for_each(|f| {
                Chart::new_with_y_range(200, 60, -5.0, 5.0, -5.0, 5.0)
                    .x_axis_style(LineStyle::Solid)
                    .y_axis_style(LineStyle::Solid)
                    .lineplot(&Shape::Continuous(Box::new(|x| {
                        self.immutable_eval_expression(x, &f.1 .1)
                    })))
                    .y_tick_display(TickDisplay::Sparse)
                    .nice();
            });
    }

    pub fn immutable_eval_expression(&self, x: f32, expr: &Expression) -> f32 {
        match expr {
            Expression::Binary(lhs, op, rhs) => match op {
                Token::Add => {
                    self.immutable_eval_expression(x, lhs) + self.immutable_eval_expression(x, rhs)
                }
                Token::Sub => {
                    self.immutable_eval_expression(x, lhs) - self.immutable_eval_expression(x, rhs)
                }
                Token::Mul => {
                    self.immutable_eval_expression(x, lhs) * self.immutable_eval_expression(x, rhs)
                }
                Token::Div => {
                    self.immutable_eval_expression(x, lhs) / self.immutable_eval_expression(x, rhs)
                }
                Token::Pow => self
                    .immutable_eval_expression(x, lhs)
                    .powf(self.immutable_eval_expression(x, rhs)),
                _ => unimplemented!(),
            },
            Expression::Identifier(_) => x,
            Expression::Number(n) => *n,
            Expression::FunctionCall(i, exprs) => {
                if self.std.get(i).is_some() {
                    let f = self.std.get(i).unwrap();
                    return f(exprs
                        .iter()
                        .map(|exp| self.immutable_eval_expression(x, exp))
                        .collect());
                } else {
                    panic!("user defined functions are not avaliable while graphing")
                }
            }
        }
    }

    pub fn eval_expression(&mut self, expr: &Expression) -> f32 {
        match expr {
            Expression::Binary(lhs, op, rhs) => match op {
                Token::Add => self.eval_expression(lhs) + self.eval_expression(rhs),
                Token::Sub => self.eval_expression(lhs) - self.eval_expression(rhs),
                Token::Mul => self.eval_expression(lhs) * self.eval_expression(rhs),
                Token::Div => self.eval_expression(lhs) / self.eval_expression(rhs),
                Token::Pow => self.eval_expression(lhs).powf(self.eval_expression(rhs)),
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
                    return f(exprs.iter().map(|x| self.eval_expression(x)).collect());
                }
                let (args, code) = self.functions.get(i).unwrap().clone();

                let scope_vars = self.scope_vars.clone();

                for (i, arg) in args.iter().enumerate() {
                    let r = self.eval_expression(&exprs[i]);
                    self.scope_vars.insert(arg.to_string(), r);
                }

                let r = self.eval_expression(&code);

                self.scope_vars = scope_vars;

                r
            }
        }
    }
}
