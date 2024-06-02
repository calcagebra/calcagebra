use std::{
    collections::HashMap,
    f32::consts::{E, PI},
};

use crate::{
    ast::{Ast, Expression},
    data::{function::Function, sizedset::SizedSet, unsizedset::UnsizedSet, Data},
    standardlibrary::StandardLibrary,
    token::Token,
};

pub type Variables = HashMap<String, Data>;
pub type Functions = HashMap<String, Data>;
pub type Std = HashMap<String, StdFunction>;
pub type StdFunction = fn(Vec<Data>, Variables, Functions, StandardLibrary) -> Data;

#[derive(Debug)]
pub struct Interpreter {
    pub variables: Variables,
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

    pub fn init_globals(&mut self) -> &Self {
        [("π", PI), ("pi", PI), ("e", E)]
            .map(|(k, v)| self.variables.insert(k.to_string(), Data::Number(v)));

        self
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
                    } else if let Data::Function(f) = self.functions.get(&i).unwrap().clone() {
                        let variables = self.variables.clone();

                        for (i, arg) in f.args.iter().enumerate() {
                            let r = Interpreter::eval_expression(
                                &exprs[i],
                                &self.variables,
                                &self.functions,
                                &self.std.map,
                            );
                            self.variables.insert(arg.to_string(), r);
                        }

                        Interpreter::eval_expression(
                            &f.expr,
                            &self.variables,
                            &self.functions,
                            &self.std.map,
                        );

                        self.variables = variables;
                    }
                }
                Ast::FunctionDeclaration(name, args, expr) => {
                    self.functions.insert(
                        name.to_string(),
                        Data::Function(Function::new(name, args.to_vec(), expr.clone())),
                    );
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
                    Token::Mod => dlhs % drhs,
                    Token::Pow => Data::Number(dlhs.to_number().powf(drhs.to_number())),
                    Token::IsEq => Data::Bool(dlhs == drhs),
                    Token::Gt => Data::Bool(dlhs > drhs),
                    Token::Lt => Data::Bool(dlhs < drhs),
                    Token::GtEq => Data::Bool(dlhs >= drhs),
                    Token::LtEq => Data::Bool(dlhs <= drhs),
                    _ => unreachable!(),
                }
            }
            Expression::Abs(operand) => {
                let data = Interpreter::eval_expression(operand, variables, functions, std);

                Data::Number(match data {
                    Data::Number(n) => n.abs(),
                    Data::Bool(b) => b as u8 as f32,
                    Data::SizedSet(s) => s.values.len() as f32,
                    Data::UnsizedSet(x) => x.len(variables, functions, std),
                    _ => unimplemented!(),
                })
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
                    variables.get(ident).unwrap().to_owned()
                } else if std.get(ident).is_some() {
                    Data::Function(Function::new(ident.to_string(), vec![], Expression::Undefined))
                } else if functions.get(ident).is_some() {
                    functions.get(ident).unwrap().to_owned()
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
            Expression::UnsizedSet(idents, conditions) => {
                Data::UnsizedSet(UnsizedSet::new(idents.to_vec(), conditions.to_vec()))
            }
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
                } else if let Data::Function(f) = functions.get(i).unwrap().clone() {
                    let mut variables = variables.clone();

                    for (i, arg) in f.args.iter().enumerate() {
                        let r = Interpreter::eval_expression(&exprs[i], &variables, functions, std);
                        variables.insert(arg.to_string(), r);
                    }

                    return Interpreter::eval_expression(&f.expr, &variables, functions, std);
                }
                unreachable!()
            }
            Expression::Differentiate(ident) => {
                let data = Interpreter::eval_expression(ident, variables, functions, std);

                match data {
                    Data::Function(f) => {
                        let r = f.expr.differentiate(&f.args);
                        Data::Function(Function::new(f.name, f.args, r))
                    }
                    _ => unreachable!(),
                }
            }
            Expression::Undefined => {
                panic!("reached undefined expression evaluation")
            }
        }
    }
}
