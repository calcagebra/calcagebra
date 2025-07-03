use std::{
	collections::HashMap,
	f32::consts::{E, PI},
};

pub type InterpreterContext<'a> = (&'a mut HashMap<String, Data>, &'a HashMap<String, Function>);

use crate::{
	ast::{AstNode, Expression},
	standardlibrary::{
		io, math,
		operators::{self, add, div, gt, gteq, is_eq, lt, lteq, mul, neq, pow, rem, sub},
	},
	token::Token,
	types::{Data, DataType},
};

#[derive(Debug, Clone)]
pub struct Interpreter {
	pub globals: HashMap<String, Data>,
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
		let mut functions = HashMap::new();

		[
			("i", Data::Number(0.0, 1.0)),
			("pi", Data::Number(PI, 0.0)),
			("π", Data::Number(PI, 0.0)),
			("e", Data::Number(E, 0.0)),
		]
		.map(|(global, data)| globals.insert(global.to_string(), data));

		[
			"print",
			"read",
			"int",
			"real",
			"add",
			"sub",
			"mul",
			"div",
			"pow",
			"rem",
			"is_eq",
			"neq",
			"gt",
			"gteq",
			"lt",
			"lteq",
			"abs",
			"round",
			"ceil",
			"floor",
			"ln",
			"log10",
			"log",
			"sin",
			"cos",
			"tan",
			"sqrt",
			"cbrt",
			"nrt",
			"graph",
			"transpose",
			"determinant",
			"adj",
			"inverse",
		]
		.map(|name| {
			functions.insert(
				name.to_string(),
				Function::STD(STDFunction {
					name: name.to_string(),
				}),
			)
		});

		Self { globals, functions }
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

				if numbertype.is_some() && number.ty() != numbertype.unwrap() {
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
				self.functions.insert(
					name,
					Function::UserDefined(UserDefinedFunction {
						params: items,
						return_type: number_type,
						code: expr,
					}),
				);
			}
		}
	}

	pub fn interpret_expression(ctx: &mut InterpreterContext, expr: &Expression) -> Data {
		match expr {
			Expression::Abs(expression) => math::abs(&Self::interpret_expression(ctx, expression)),
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
				if let Data::Number(condition, _) = Self::interpret_expression(ctx, condition) {
					return if condition != 0.0 {
						Self::interpret_expression(ctx, then)
					} else {
						Self::interpret_expression(ctx, otherwise)
					};
				}

				panic!("expected number in condition for branch statement")
			}
			Expression::Identifier(name) => {
				// TODO: Error handling for when name does not
				ctx.0.get(name).unwrap().to_owned()
			}
			Expression::Float(f) => Data::Number(*f, 0.0),
			Expression::Matrix(matrix) => Data::Matrix(
				matrix
					.iter()
					.map(|f| {
						f.iter()
							.map(|g| Self::interpret_expression(ctx, g))
							.collect::<Vec<Data>>()
					})
					.collect::<Vec<Vec<Data>>>(),
			),
			Expression::FunctionCall(name, exprs) => {
				if ctx.1.contains_key(name) {
					let f = ctx.1.get(name).unwrap();

					if let Function::UserDefined(g) = f {
						for (i, (arg, numbertype)) in g.params.iter().enumerate() {
							let r = Self::interpret_expression(ctx, &exprs[i]);

							if r.ty() != *numbertype {
								// TODO: error handling
								panic!("type mismatch")
							}

							ctx.0.insert(arg.to_string(), r);
						}

						return g.execute(ctx);
					} else if let Function::STD(g) = f {
						return g.execute(ctx, exprs);
					}

					unreachable!()
				}

				panic!("undefined function")
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum Function {
	UserDefined(UserDefinedFunction),
	STD(STDFunction),
}

#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
	pub params: Vec<(String, DataType)>,
	pub return_type: DataType,
	pub code: Expression,
}

impl UserDefinedFunction {
	pub fn execute(&self, ctx: &mut InterpreterContext) -> Data {
		Interpreter::interpret_expression(ctx, &self.code)
	}
}

#[derive(Debug, Clone)]
pub struct STDFunction {
	pub name: String,
}

impl STDFunction {
	pub fn execute(&self, ctx: &mut InterpreterContext, exprs: &Vec<Expression>) -> Data {
		if &self.name == "graph" {
			return math::graph(&exprs[0], ctx);
		}

		let mut args = vec![];

		for expr in exprs {
			args.push(Interpreter::interpret_expression(ctx, expr))
		}

		match self.name.as_str() {
			"print" => io::print(args),
			"read" => io::read(ctx),
			"add" => operators::add(&args[0], &args[1]),
			"sub" => operators::sub(&args[0], &args[1]),
			"mul" => operators::mul(&args[0], &args[1]),
			"div" => operators::div(&args[0], &args[1]),
			"pow" => operators::pow(&args[0], &args[1]),
			"rem" => operators::rem(&args[0], &args[1]),
			"is_eq" => operators::is_eq(&args[0], &args[1]),
			"neq" => operators::neq(&args[0], &args[1]),
			"gt" => operators::gt(&args[0], &args[1]),
			"gteq" => operators::gteq(&args[0], &args[1]),
			"lt" => operators::lt(&args[0], &args[1]),
			"lteq" => operators::lteq(&args[0], &args[1]),
			"abs" => math::abs(&args[0]),
			"round" => math::round(&args[0]),
			"ceil" => math::ceil(&args[0]),
			"floor" => math::floor(&args[0]),
			"ln" => math::ln(&args[0]),
			"log10" => math::log10(&args[0]),
			"log" => math::log(&args[0], &args[1]),
			"sin" => math::sin(&args[0]),
			"cos" => math::cos(&args[0]),
			"tan" => math::tan(&args[0]),
			"sqrt" => math::sqrt(&args[0]),
			"nrt" => math::nrt(&args[0], &args[1]),
			"transpose" => math::transpose(&args[0]),
			"determinant" => math::determinant(&args[0]),
			"adj" => math::adj(&args[0]),
			"inverse" => math::inverse(&args[0]),
			_ => unreachable!(),
		}
	}
}
