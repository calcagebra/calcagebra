use std::{
	collections::HashMap,
	f32::consts::{E, PI},
};

use crate::{
	ast::{AstNode, Expression},
	standardlibrary::{
		call, ctx_call, is_std, math, needs_ctx,
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

impl Default for Interpreter {
	fn default() -> Self {
		Self::new()
	}
}

impl Interpreter {
	pub fn new() -> Self {
		let mut globals = HashMap::new();

		[
			("i", Number::Complex(0.0, 1.0)),
			("pi", Number::Real(PI)),
			("π", Number::Real(PI)),
			("e", Number::Real(E)),
		]
		.map(|(global, data)| globals.insert(global.to_string(), data));

		Self {
			globals,
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
			AstNode::Assignment((name, numbertype), expr) => {
				let number = Self::interpret_expression(&mut (&mut self.globals, &self.functions), &expr);

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
				Self::interpret_expression(
					&mut (&mut self.globals, &self.functions),
					&Expression::FunctionCall(name, exprs),
				);
			}
			AstNode::FunctionDeclaration(name, items, number_type, expr) => {
				self
					.functions
					.insert(name, Function::new(items, number_type, expr));
			}
		}
	}

	pub fn interpret_expression(
		ctx: &mut (&mut HashMap<String, Number>, &HashMap<String, Function>),
		expr: &Expression,
	) -> Number {
		match expr {
			Expression::Abs(expression) => math::abs(vec![Self::interpret_expression(ctx, expression)]),
			Expression::Binary(lhs, token, rhs) => {
				let lhd = &Self::interpret_expression(ctx, lhs);

				let rhd = &Self::interpret_expression(ctx, rhs);

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
				let condition = Self::interpret_expression(ctx, condition).real();

				if condition != 0.0 {
					Self::interpret_expression(ctx, then)
				} else {
					Self::interpret_expression(ctx, otherwise)
				}
			}
			Expression::Identifier(name) => {
				// TODO: Error handling for when name does not
				ctx.0.get(name).unwrap().to_owned()
			}
			Expression::Real(f) => Number::Real(*f),
			Expression::Integer(i) => Number::Int(*i),
			Expression::Matrix(matrix) => Number::Matrix(
				matrix
					.iter()
					.map(|f| {
						f.iter()
							.map(|g| Self::interpret_expression(ctx, g))
							.collect::<Vec<Number>>()
					})
					.collect::<Vec<Vec<Number>>>(),
			),
			Expression::FunctionCall(name, exprs) => {
				if is_std(name) && !needs_ctx(name) {
					let mut args = vec![];

					for expr in exprs {
						args.push(Self::interpret_expression(ctx, expr))
					}

					return call(name, args);
				} else if is_std(name) && needs_ctx(name) {
					return ctx_call(name, exprs, ctx);
				} else if ctx.1.contains_key(name) {
					let f = ctx.1.get(name).unwrap();

					for (i, (arg, numbertype)) in f.params.iter().enumerate() {
						let r = Self::interpret_expression(ctx, &exprs[i]);

						if r.r#type() != *numbertype {
							// TODO: error handling
							panic!("type mismatch")
						}

						ctx.0.insert(arg.to_string(), r);
					}

					return f.execute(ctx);
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

	pub fn execute(
		&self,
		ctx: &mut (&mut HashMap<String, Number>, &HashMap<String, Function>),
	) -> Number {
		Interpreter::interpret_expression(ctx, &self.code)
	}
}
