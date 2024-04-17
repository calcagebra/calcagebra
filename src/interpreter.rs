use std::{
    collections::HashMap,
    f32::consts::{E, PI},
};

use crate::{
    ast::{Ast, Expression},
    data::{sizedset::SizedSet, Data},
    standardlibrary::StandardLibrary,
    token::Token,
};

pub type Variables = HashMap<String, Data>;
pub type Functions = HashMap<String, (Vec<String>, Expression)>;
pub type Std = HashMap<String, Function>;
pub type Function = fn(Vec<Data>, Variables, Functions, StandardLibrary) -> Data;

#[derive(Debug)]
pub struct Interpreter {
    variables: Variables,
    functions: Functions,
    std: StandardLibrary,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            std: StandardLibrary::new(),
        }
    }

    pub fn init_globals(&mut self) {
        [("pi", PI), ("e", E)].map(|(k, v)| self.variables.insert(k.to_string(), Data::Number(v)));
    }

    pub fn run(&mut self, ast: Vec<Ast>) {
        self.std.init_std();
        self.init_globals();
        for node in ast {
            match node {
                Ast::Assignment(name, expr) => {
                    let r = Interpreter::eval_expression(
                        &expr,
                        &self.variables,
                        &self.functions,
                        &self.std.map,
                    );
                    self.variables.insert(name.to_string(), r);
                }
                Ast::FunctionCall(i, exprs) => {
                    if self.std.map.get(&i).is_some() {
                        let f = self.std.map.get(&i).unwrap();
                        f(
                            exprs
                                .iter()
                                .map(|x| {
                                    Interpreter::eval_expression(
                                        x,
                                        &self.variables,
                                        &self.functions,
                                        &self.std.map,
                                    )
                                })
                                .collect(),
                            self.variables.clone(),
                            self.functions.clone(),
                            self.std.clone(),
                        );
                    } else {
                        let (args, code) = self.functions.get(&i).unwrap().clone();

                        let variables = self.variables.clone();

                        for (i, arg) in args.iter().enumerate() {
                            let r = Interpreter::eval_expression(
                                &exprs[i],
                                &self.variables,
                                &self.functions,
                                &self.std.map,
                            );
                            self.variables.insert(arg.to_string(), r);
                        }

                        Interpreter::eval_expression(
                            &code,
                            &self.variables,
                            &self.functions,
                            &self.std.map,
                        );

                        self.variables = variables;
                    }
                }
                Ast::FunctionDeclaration(name, args, code) => {
                    self.functions
                        .insert(name.to_string(), (args.to_vec(), code.clone()));
                }
            }
        }
    }

    pub fn eval_expression(
        expr: &Expression,
        variables: &Variables,
        functions: &Functions,
        std: &Std,
    ) -> Data {
        match expr {
            Expression::Binary(lhs, op, rhs) => {
                let dlhs = Interpreter::eval_expression(lhs, variables, functions, std);
                let drhs = Interpreter::eval_expression(rhs, variables, functions, std);
                match op {
                    Token::Add => dlhs + drhs,
                    Token::Sub => dlhs - drhs,
                    Token::Mul => dlhs * drhs,
                    Token::Div => dlhs / drhs,
                    Token::Modulo => dlhs % drhs,
                    Token::Pow => Data::Number(dlhs.to_number().powf(drhs.to_number())),
                    Token::IsEq => Data::Bool(dlhs == drhs),
                    Token::Gt => Data::Bool(dlhs > drhs),
                    Token::Lt => Data::Bool(dlhs < drhs),
                    Token::GtEq => Data::Bool(dlhs >= drhs),
                    Token::LtEq => Data::Bool(dlhs <= drhs),
                    _ => unimplemented!(),
                }
            }
            Expression::Branched(condition, a, b) => {
                if Interpreter::eval_expression(condition, variables, functions, std).to_bool() {
                    Interpreter::eval_expression(a, variables, functions, std)
                } else {
                    Interpreter::eval_expression(b, variables, functions, std)
                }
            }
            Expression::Identifier(ident) => {
                if variables.get(ident).is_some() {
                    variables.get(ident).unwrap().clone()
                } else if std.get(ident).is_some() || functions.get(ident).is_some() {
                    Data::Function(ident.to_string())
                } else {
                    panic!("attempt to access value of not assigned identifier `{ident}`")
                }
            }
            Expression::Number(n) => Data::Number(*n),
            Expression::SizedSet(s) => Data::SizedSet(SizedSet::new(
                s.iter()
                    .map(|f| Interpreter::eval_expression(f, variables, functions, std))
                    .collect(),
            )),
            Expression::FunctionCall(i, exprs) => {
                if std.get(i).is_some() {
                    let f = std.get(i).unwrap();
                    return f(
                        exprs
                            .iter()
                            .map(|x| Interpreter::eval_expression(x, variables, functions, std))
                            .collect(),
                        variables.clone(),
                        functions.clone(),
                        StandardLibrary::from_map(std.clone()),
                    );
                }
                let (args, code) = functions.get(i).unwrap().clone();

                let mut variables = variables.clone();

                for (i, arg) in args.iter().enumerate() {
                    let r = Interpreter::eval_expression(&exprs[i], &variables, functions, std);
                    variables.insert(arg.to_string(), r);
                }

                Interpreter::eval_expression(&code, &variables, functions, std)
            }
        }
    }
}
