use std::{
	collections::HashMap,
	f32::consts::{E, PI},
};

use crate::{
	ast::{AstNode, Expression},
	standardlibrary::{
		call, ctx_call, is_std, needs_ctx,
		operands::{add, div, gt, gteq, is_eq, lt, lteq, mul, neq, pow, rem, sub},
	},
	token::Token,
	types::{Number, NumberType},
};

#[derive(Debug, Clone)]
pub struct Interpreter {
	pub globals: HashMap<String, Number>,
	pub functions: HashMap<String, Function>,
}

impl Interpreter {
	pub fn new() -> Self {
		Self {
			globals: HashMap::new(),
			functions: HashMap::new(),
		}
	}

	pub fn setup(&mut self) {
		let globals = vec![
			("i", Number::Complex(0.0, 1.0)),
			("pi", Number::Real(PI)),
			("π", Number::Real(PI)),
			("e", Number::Real(E)),
		];

		for (global, data) in globals {
			self.globals.insert(String::from(global), data);
		}
	}

	pub fn interpret(&mut self, ast: Vec<AstNode>) {
		self.setup();

		for node in ast {
			self.interpret_node(node);
		}
	}

	pub fn interpret_node(&mut self, node: AstNode) {
		match node {
			AstNode::Assignment((name, numbertype), expr) => {
				let number = self.interpret_expression(&expr);

				if numbertype.is_some() && number.r#type() != numbertype.unwrap() {
					// TODO: proper errors
					panic!(
						"type mismatch found {} expected {}",
						number,
						numbertype.unwrap()
					)
				}

				self.globals.insert(name, number);
			}
			AstNode::FunctionCall(name, exprs) => {
				if is_std(&name) && !needs_ctx(&name) {
					let mut args = vec![];

					for expr in exprs {
						args.push(self.interpret_expression(&expr))
					}

					call(&name, args);
				} else if is_std(&name) && needs_ctx(&name) {
					ctx_call(&name, exprs, self);
				} else if self.functions.contains_key(&name) {
					let f: Function = self.functions.get(&name).unwrap().clone();
					let globals = self.globals.clone();

					for (i, (arg, numbertype)) in f.params.iter().enumerate() {
						let r = self.interpret_expression(&exprs[i]);

						if r.r#type() != *numbertype {
							// TODO: error handling
							panic!("type mismatch")
						}

						self.globals.insert(arg.to_string(), r);
					}

					self.interpret_expression(&f.code);

					self.globals = globals;
				}
			}
			AstNode::FunctionDeclaration(name, items, number_type, expr) => {
				self
					.functions
					.insert(name, Function::new(items, number_type, expr));
			}
		}
	}

	pub fn interpret_expression(&mut self, expr: &Expression) -> Number {
		match expr {
			Expression::Abs(expression) => {
				let number = self.interpret_expression(expression);

				let numbertype = number.r#type();

				match numbertype {
					NumberType::Int => Number::Int(number.int().abs()),
					NumberType::Real => Number::Real(number.real().abs()),
					NumberType::Complex => {
						Number::Real(number.array().iter().map(|f| f * f).sum::<f32>().sqrt())
					}
					_ => todo!(),
				}
			}
			Expression::Binary(lhs, token, rhs) => {
				let lhd = &self.interpret_expression(lhs);

				let rhd = &self.interpret_expression(rhs);

				match token {
					Token::Add => add(lhd, rhd),
					Token::Sub => sub(lhd, rhd),
					Token::Mul => mul(lhd, rhd),
					Token::Div => div(lhd, rhd),
					Token::Pow => pow(lhd, rhd),
					Token::Rem => rem(lhd, rhd),
					Token::IsEq => is_eq(lhd, rhd),
					Token::NEq => neq(lhd, rhd),
					Token::Gt => gt(lhd, rhd),
					Token::GtEq => gteq(lhd, rhd),
					Token::Lt => lt(lhd, rhd),
					Token::LtEq => lteq(lhd, rhd),
					_ => unreachable!(),
				}
			}
			Expression::Branched(condition, then, otherwise) => {
				let condition = self.interpret_expression(condition).real();

				if condition != 0.0 {
					self.interpret_expression(then)
				} else {
					self.interpret_expression(otherwise)
				}
			}
			Expression::Identifier(name) => {
				// TODO: Error handling for when name does not
				self.globals.get(name).unwrap().clone()
			}
			Expression::Real(f) => Number::Real(*f),
			Expression::Integer(i) => Number::Int(*i),
			Expression::Matrix(matrix) => Number::Matrix(
				matrix
					.iter()
					.map(|f| {
						f.iter()
							.map(|g| self.interpret_expression(g))
							.collect::<Vec<Number>>()
					})
					.collect::<Vec<Vec<Number>>>(),
			),
			Expression::FunctionCall(name, exprs) => {
				// A simple standard function is a term used to define a function which
				// takes only Number as arguments opposed to say function name
				if is_std(&name) && !needs_ctx(&name) {
					let mut args = vec![];

					for expr in exprs {
						args.push(self.interpret_expression(&expr))
					}

					return call(&name, args);
				}
				// A complex function is one which may take any combination of argument and types
				// currently only graph is a complex function
				else if is_std(&name) && needs_ctx(&name) {
					return ctx_call(&name, exprs.to_vec(), self);
				} else if self.functions.contains_key(name) {
					let f = self.functions.get(name).unwrap().clone();
					let globals = self.globals.clone();

					for (i, (arg, numbertype)) in f.params.iter().enumerate() {
						let r = self.interpret_expression(&exprs[i]);

						if r.r#type() != *numbertype {
							// TODO: error handling
							panic!("type mismatch")
						}

						self.globals.insert(arg.to_string(), r);
					}

					let r = self.interpret_expression(&f.code);

					self.globals = globals;

					if r.r#type() != f.return_type {
						panic!("return type and expression type are not the same")
					}

					return r;
				}

				unreachable!()
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct Function {
	pub params: Vec<(String, NumberType)>,
	pub return_type: NumberType,
	pub code: Expression,
}

impl Function {
	pub fn new(params: Vec<(String, NumberType)>, return_type: NumberType, code: Expression) -> Self {
		Self {
			params,
			return_type,
			code,
		}
	}
}
