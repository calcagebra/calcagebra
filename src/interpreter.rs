use std::{collections::HashMap, ops::Rem};

use crate::{
	ast::{AstNode, Expression},
	standardlibrary::{
		complex_call, is_complex_standard_function, is_simple_standard_function, simple_call,
	},
	token::Token,
	types::{Number, NumberType},
};

#[derive(Debug)]
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

	pub fn interpret(&mut self, ast: Vec<AstNode>) {
		for node in ast {
			self.interpret_node(node);
		}
	}

	pub fn interpret_node(&mut self, node: AstNode) {
		match node {
			AstNode::Import(_) => todo!(),
			AstNode::Assignment((name, numbertype), expr) => {
				let number = self.interpret_expression(&expr);

				if number.r#type() != numbertype {
					// TODO: proper errors
					panic!("type mismatch")
				}

				self.globals.insert(name, number);
			}
			AstNode::FunctionCall(name, exprs) => {
				// A simple standard function is a term used to define a function which
				// takes only Number as arguments opposed to say function name
				if is_simple_standard_function(&name) {
					let mut args = vec![];

					for expr in exprs {
						args.push(self.interpret_expression(&expr))
					}

					simple_call(&name, args);
				}
				// A complex function is one which may take any combination of argument and types
				// currently only graph is a complex function
				else if is_complex_standard_function(&name) {
					complex_call(&name, exprs, self);
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
					.insert(name.clone(), Function::new(name, items, number_type, expr));
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
				}
			}
			Expression::Binary(lhs, token, rhs) => {
				let lhd = self.interpret_expression(lhs);

				let rhd = self.interpret_expression(rhs);

				match token {
					Token::Add => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() + rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f32 + rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() + rhd.int() as f32);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() + rhd.real()),
					},
					Token::Sub => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() - rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f32 - rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() - rhd.int() as f32);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() - rhd.real()),
					},
					Token::Mul => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() * rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f32 * rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() * rhd.int() as f32);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() * rhd.real()),
					},
					Token::Div => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() / rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f32 / rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() / rhd.int() as f32);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() / rhd.real()),
					},
					Token::Pow => {
						match (lhd.r#type(), rhd.r#type()) {
							(NumberType::Int, NumberType::Int) => {
								// TODO: Handle negative errors
								return Number::Int(lhd.int().pow(rhd.int().try_into().unwrap()));
							}
							(NumberType::Int, NumberType::Real) => {
								return Number::Real((lhd.int() as f32).powf(rhd.real()));
							}
							(NumberType::Real, NumberType::Int) => {
								return Number::Real(lhd.real().powf(rhd.int() as f32));
							}
							(NumberType::Real, NumberType::Real) => {
								return Number::Real(lhd.real().powf(rhd.real()));
							}
						}
					}
					Token::Rem => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int().rem(rhd.int())),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real((lhd.int() as f32).rem(rhd.real()));
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real((lhd.real()).rem(rhd.int() as f32));
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Real(lhd.real().rem(rhd.real()));
						}
					},
					Token::IsEq => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => {
							return Number::Int((lhd.int() == rhd.int()) as i32);
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Int((lhd.real() == rhd.real()) as i32);
						}
						_ => unimplemented!(),
					},
					Token::NEq => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => {
							return Number::Int((lhd.int() != rhd.int()) as i32);
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Int((lhd.real() != rhd.real()) as i32);
						}
						_ => unimplemented!(),
					},
					Token::Gt => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => {
							return Number::Int((lhd.int() > rhd.int()) as i32);
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Int((lhd.real() > rhd.real()) as i32);
						}
						_ => unimplemented!(),
					},
					Token::GtEq => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => {
							return Number::Int((lhd.int() >= rhd.int()) as i32);
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Int((lhd.real() >= rhd.real()) as i32);
						}
						_ => unimplemented!(),
					},
					Token::Lt => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => {
							return Number::Int((lhd.int() < rhd.int()) as i32);
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Int((lhd.real() < rhd.real()) as i32);
						}
						_ => unimplemented!(),
					},
					Token::LtEq => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => {
							return Number::Int((lhd.int() <= rhd.int()) as i32);
						}
						(NumberType::Real, NumberType::Real) => {
							return Number::Int((lhd.real() <= rhd.real()) as i32);
						}
						_ => unimplemented!(),
					},
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
				*self.globals.get(name).unwrap()
			}
			Expression::Real(f) => Number::Real(*f),
			Expression::Integer(i) => Number::Int(*i),
			Expression::FunctionCall(name, exprs) => {
				// A simple standard function is a term used to define a function which
				// takes only Number as arguments opposed to say function name
				if is_simple_standard_function(&name) {
					let mut args = vec![];

					for expr in exprs {
						args.push(self.interpret_expression(&expr))
					}

					return simple_call(&name, args);
				}
				// A complex function is one which may take any combination of argument and types
				// currently only graph is a complex function
				else if is_complex_standard_function(&name) {
					return complex_call(&name, exprs.to_vec(), self);
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

					return r;
				}

				unreachable!()
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct Function {
	pub name: String,
	pub params: Vec<(String, NumberType)>,
	pub return_type: NumberType,
	pub code: Expression,
}

impl Function {
	pub fn new(
		name: String,
		params: Vec<(String, NumberType)>,
		return_type: NumberType,
		code: Expression,
	) -> Self {
		Self {
			name,
			params,
			return_type,
			code,
		}
	}
}
