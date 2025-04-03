use std::{collections::HashMap, ops::Rem};

use crate::{
	ast::{AstNode, Expression},
	token::Token,
	types::{Number, NumberType},
};

#[derive(Debug)]
pub struct Interpreter {
	globals: HashMap<String, Number>,
}

impl Interpreter {
	pub fn new() -> Self {
		Self {
			globals: HashMap::new(),
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
				let number = self.interpret_expression(expr);

				if number.r#type() != numbertype {
					// TODO: proper errors
					panic!("type mismatch")
				}

				self.globals.insert(name, number);
			}
			AstNode::FunctionCall(_, expressions) => todo!(),
			AstNode::FunctionDeclaration(_, items, number_type, expression) => todo!(),
		}
	}

	pub fn interpret_expression(&self, expr: Expression) -> Number {
		match expr {
			Expression::Abs(expression) => {
				let number = self.interpret_expression(*expression);

				let numbertype = number.r#type();

				match numbertype {
					NumberType::Int => Number::Int(number.int().abs()),
					NumberType::Real => Number::Real(number.real().abs()),
				}
			}
			Expression::Binary(lhs, token, rhs) => {
				let lhd = self.interpret_expression(*lhs);

				let rhd = self.interpret_expression(*rhs);

				match token {
					Token::Add => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() + rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f64 + rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() + rhd.int() as f64);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() + rhd.real()),
					},
					Token::Sub => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() - rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f64 - rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() - rhd.int() as f64);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() - rhd.real()),
					},
					Token::Mul => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() * rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f64 * rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() * rhd.int() as f64);
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real() * rhd.real()),
					},
					Token::Div => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int() / rhd.int()),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real(lhd.int() as f64 / rhd.real());
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real(lhd.real() / rhd.int() as f64);
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
								return Number::Real((lhd.int() as f64).powf(rhd.real()));
							}
							(NumberType::Real, NumberType::Int) => {
								return Number::Real(lhd.real().powf(rhd.int() as f64));
							}
							(NumberType::Real, NumberType::Real) => {
								return Number::Real(lhd.real().powf(rhd.real()));
							}
						}
					}
					Token::Rem => match (lhd.r#type(), rhd.r#type()) {
						(NumberType::Int, NumberType::Int) => return Number::Int(lhd.int().rem(rhd.int())),
						(NumberType::Int, NumberType::Real) => {
							return Number::Real((lhd.int() as f64).rem(rhd.real()));
						}
						(NumberType::Real, NumberType::Int) => {
							return Number::Real((lhd.real()).rem(rhd.int() as f64));
						}
						(NumberType::Real, NumberType::Real) => return Number::Real(lhd.real().rem(rhd.real())),
					},
					_ => unreachable!(),
				}
			}
			Expression::Branched(condition, then, otherwise) => {
                if 
            },
			Expression::Identifier(_) => todo!(),
			Expression::Real(_) => todo!(),
			Expression::Integer(_) => todo!(),
			Expression::FunctionCall(_, expressions) => todo!(),
		}
	}
}
