use crate::standardlibrary::{
	internal_type_map, is_std, math,
	operators::{add, div, gt, gteq, is_eq, lt, lteq, mul, neq, pow, rem, sub},
};
use crate::{
	errors::{Error, TypeError},
	interpreter::{Function, InterpreterContext, UserDefinedFunction, Variable},
	token::Token,
	types::{Data, DataType},
};
use rust_decimal::{Decimal, MathematicalOps};
use std::{fmt::Display, ops::Range};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	Assignment((String, Option<DataType>), Box<Expression>),
	Abs(Box<Expression>),
	Binary(Box<Expression>, Token, Box<Expression>),
	Branched(Box<Expression>, Box<Expression>, Box<Expression>),
	Identifier(String),
	Float(Decimal),
	Matrix(Vec<Vec<Expression>>),
	FunctionCall(String, Vec<(Expression, Range<usize>)>),
	FunctionDeclaration(
		String,
		Vec<(String, DataType)>,
		DataType,
		Box<Expression>,
		Range<usize>,
	),
}

impl Expression {
	#[inline(always)]
	pub fn evaluate<'a, 'b>(
		self,
		ctx: &'a mut InterpreterContext<'b>,
		range: Range<usize>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		match self {
			Expression::Assignment((name, numbertype), expr) => {
				let number = expr.evaluate(ctx, range.clone())?;

				if let Some(ty) = numbertype
					&& number.ty() != numbertype.unwrap()
				{
					return Err(TypeError::new(ty, number.ty(), 0..0).to_error());
				}

				ctx.0.insert(name, Variable::new(number.clone(), true));

				Ok(number)
			}
			Expression::FunctionDeclaration(name, items, number_type, expr, range) => {
				ctx.1.insert(
					name.to_owned(),
					Function::UserDefined(UserDefinedFunction {
						params: items,
						return_type: number_type,
						code: *expr,
						range,
					}),
				);

				Ok(Data::Ident(name))
			}
			Expression::Abs(expression) => {
				let data = expression.evaluate(ctx, range.clone())?;
				Ok(math::abs(&data))
			}
			Expression::Binary(lhs, token, rhs) => {
				let lhd = lhs.evaluate(ctx, range.clone())?;
				let rhd = rhs.evaluate(ctx, range.clone())?;

				Ok(match token {
					Token::Add => add(&lhd, &rhd),
					Token::Sub => sub(&lhd, &rhd),
					Token::Mul => mul(&lhd, &rhd),
					Token::Div => div(&lhd, &rhd),
					Token::Pow => pow(&lhd, &rhd),
					Token::Rem => rem(&lhd, &rhd),
					Token::IsEq => is_eq(&lhd, &rhd),
					Token::NEq => neq(&lhd, &rhd),
					Token::Gt => gt(&lhd, &rhd),
					Token::GtEq => gteq(&lhd, &rhd),
					Token::Lt => lt(&lhd, &rhd),
					Token::LtEq => lteq(&lhd, &rhd),
					_ => unreachable!(),
				})
			}
			Expression::Branched(condition, then, otherwise) => {
				let data = condition.evaluate(ctx, range.clone())?;

				if let Data::Number(condition, _) = data {
					return if condition != Decimal::ZERO {
						Ok(then.evaluate(ctx, range.clone())?)
					} else {
						Ok(otherwise.evaluate(ctx, range.clone())?)
					};
				}

				Err(TypeError::new(DataType::Number, data.ty(), 0..0).to_error())
			}
			Expression::Identifier(name) => {
				if ctx.0.contains_key(&name) {
					Ok(ctx.0.get(&name).unwrap().to_owned().value)
				} else {
					Ok(Data::Ident(name))
				}
			}
			Expression::Float(f) => Ok(Data::new_real(f)),
			Expression::Matrix(matrix) => {
				let mut matrix_data = vec![];

				for row in matrix {
					let mut row_data = vec![];
					for element in row {
						let data = element.evaluate(ctx, range.clone())?;

						row_data.push(data);
					}
					matrix_data.push(row_data);
				}

				Ok(Data::Matrix(matrix_data))
			}
			Expression::FunctionCall(name, exprs) => {
				if !ctx.1.contains_key(&name) {
					return Err(Error::LogicError(format!("undefined function: `{name}`")));
				}

				let f = ctx.1.get(&name).unwrap().clone();

				let mut args = vec![];

				for (expr, range) in exprs {
					let data = expr.evaluate(ctx, range)?;
					args.push(data);
				}

				f.execute(ctx, args)
			}
		}
	}

	#[inline(always)]
	pub fn infer_datatype(&self) -> Option<DataType> {
		match self {
			Expression::Abs(expression) => expression.infer_datatype(),
			Expression::Branched(_, e1, _) => e1.infer_datatype(),
			Expression::Binary(lhs, _, rhs) => {
				let lhs = Self::infer_datatype(lhs);
				let rhs = Self::infer_datatype(rhs);

				if lhs.is_none() || rhs.is_none() {
					return None;
				};

				let lhs = lhs.unwrap();
				let rhs = rhs.unwrap();

				match (lhs, rhs) {
					(DataType::Number, DataType::Number) => Some(DataType::Number),
					(DataType::Matrix, _) | (_, DataType::Matrix) => Some(DataType::Matrix),
					_ => None,
				}
			}
			Expression::Identifier(_) => None,
			Expression::Float(..) => Some(DataType::Number),
			Expression::Matrix(..) => Some(DataType::Matrix),
			Expression::FunctionCall(ident, _) => {
				if is_std(ident) {
					Some(internal_type_map(ident).1)
				} else {
					None
				}
			}
			Expression::Assignment(_, expression) => expression.infer_datatype(),
			Expression::FunctionDeclaration(..) => Some(DataType::Ident),
		}
	}

	#[inline(always)]
	pub fn differentiate<'a, 'b>(
		&self,
		wrt: &Data,
		ctx: &'a mut InterpreterContext<'b>,
	) -> Result<Expression, Error>
	where
		'b: 'a,
	{
		let Data::Ident(name) = wrt else {
			return Err(Error::LogicError(
				"expected ident to differetiate with".to_string(),
			));
		};

		Ok(
			match self {
				t @ Expression::Binary(e1, op, e2) => match op {
					Token::Add | Token::Sub => Expression::Binary(
						Box::new(e1.differentiate(wrt, ctx)?),
						op.to_owned(),
						Box::new(e2.differentiate(wrt, ctx)?),
					),
					Token::Mul => Expression::Binary(
						Box::new(Expression::Binary(
							Box::new(e1.differentiate(wrt, ctx)?),
							Token::Mul,
							e2.to_owned(),
						)),
						Token::Add,
						Box::new(Expression::Binary(
							e1.to_owned(),
							Token::Mul,
							Box::new(e2.differentiate(wrt, ctx)?),
						)),
					),
					Token::Div => Expression::Binary(
						Box::new(Expression::Binary(
							Box::new(Expression::Binary(
								Box::new(e1.differentiate(wrt, ctx)?),
								Token::Mul,
								e2.to_owned(),
							)),
							Token::Sub,
							Box::new(Expression::Binary(
								e1.to_owned(),
								Token::Mul,
								Box::new(e2.differentiate(wrt, ctx)?),
							)),
						)),
						Token::Div,
						Box::new(Expression::Binary(
							e2.to_owned(),
							Token::Pow,
							Box::new(Expression::Float(Decimal::TWO)),
						)),
					),
					Token::Pow => match (*e1.to_owned(), *e2.to_owned()) {
						(e @ Expression::Float(..), Expression::Identifier(ident)) => {
							if name == &ident {
								Expression::Binary(
									Box::new(Expression::FunctionCall("ln".to_string(), vec![(e, 0..0)])),
									Token::Mul,
									Box::new(t.to_owned()),
								)
							} else {
								Expression::Float(Decimal::ZERO)
							}
						}
						(Expression::Identifier(ident), Expression::Float(n)) => {
							if name == &ident {
								Expression::Binary(
									e2.to_owned(),
									Token::Mul,
									Box::new(Expression::Binary(
										e1.to_owned(),
										Token::Pow,
										Box::new(Expression::Float(n - Decimal::ONE)),
									)),
								)
							} else {
								Expression::Float(Decimal::ZERO)
							}
						},
						// TODO: Add case for f(x)^g(x)
						_ => unimplemented!(),
					},
					_ => unimplemented!(),
				},
				Expression::Branched(_, _, _) => todo!(),
				Expression::Identifier(ident) => {
					let Data::Ident(name) = wrt else {
						return Err(Error::LogicError(
							"expected variable to differentiate with respect to".to_string(),
						));
					};
					if ident != name {
						Expression::Float(Decimal::ZERO)
					} else {
						Expression::Float(Decimal::ONE)
					}
				}
				Expression::Float(_) => Expression::Float(Decimal::ZERO),
				Expression::FunctionCall(name, args) => {
					let func = &ctx.1.get(name);

					if func.is_none() {
						return Err(Error::LogicError(
							"expected function to differentiate".to_string(),
						));
					}

					let func = func.unwrap().clone();

					let Data::Expression(mut expr) = func.differentiate(wrt, args, ctx)? else {
						unreachable!()
					};

					for (arg, _) in args {
						expr = Expression::Binary(
							Box::new(expr),
							Token::Mul,
							Box::new(arg.differentiate(wrt, ctx)?),
						)
					}

					expr
				}
				_ => unimplemented!(),
			}
			.simplify(),
		)
	}

	#[inline(always)]
	pub fn simplify(&self) -> Expression {
		match self {
			Expression::Binary(e1, op, e2) => match (*e1.to_owned(), op, *e2.to_owned()) {
				(Expression::Float(a), Token::Add, Expression::Float(b)) => Expression::Float(a + b),
				(Expression::Float(a), Token::Sub, Expression::Float(b)) => Expression::Float(a - b),
				(Expression::Float(a), Token::Mul, Expression::Float(b)) => Expression::Float(a * b),
				(Expression::Float(a), Token::Div, Expression::Float(b)) => Expression::Float(a / b),
				(Expression::Float(a), Token::Pow, Expression::Float(b)) => Expression::Float(a.powd(b)),
				(Expression::Float(a), Token::Mul, Expression::Binary(b, Token::Mul, c)) => {
					match (*b.to_owned(), *c.to_owned()) {
						(Expression::Float(x), Expression::Float(y)) => Expression::Float(a * x * y),
						(Expression::Float(x), d) => {
							Expression::Binary(Box::new(Expression::Float(x * a)), Token::Mul, Box::new(d))
						}
						(d, Expression::Float(x)) => {
							Expression::Binary(Box::new(Expression::Float(x * a)), Token::Mul, Box::new(d))
						}
						_ => Expression::Binary(
							Box::new(Expression::Float(a)),
							Token::Mul,
							Box::new(Expression::Binary(b, Token::Mul, c)),
						),
					}
				}
				_ => Expression::Binary(
					Box::new(e1.simplify()),
					op.to_owned(),
					Box::new(e2.simplify()),
				),
			},
			_ => self.to_owned(),
		}
	}
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Expression::Assignment((name, ty), expr) => format!(
					"let {name}{} = {expr}",
					if ty.is_some() {
						format!(": {}", ty.unwrap())
					} else {
						String::new()
					}
				),
				Expression::FunctionDeclaration(name, params, return_type, expr, _) => format!(
					"fn {name}({}): {return_type} = {expr}",
					params
						.iter()
						.map(|(name, ty)| format!("{name}: {ty}"))
						.collect::<Vec<String>>()
						.join(",")
				),
				Expression::Abs(expr) => format!("|{expr}|"),
				Expression::Binary(e1, op, e2) => {
					if *op == Token::Mul {
						if Expression::Float(Decimal::ZERO) == *e1.to_owned()
							|| Expression::Float(Decimal::ZERO) == *e2.to_owned()
						{
							String::new()
						} else if Expression::Float(Decimal::ONE) == *e1.to_owned() {
							format!("{e2}")
						} else if Expression::Float(Decimal::ONE) == *e2.to_owned() {
							format!("{e1}")
						} else if let Expression::Float(t) = *e2.to_owned() {
							format!("{t}{e1}")
						} else if let Expression::Float(t) = *e1.to_owned() {
							format!("{t}{e2}")
						} else {
							format!("{e1}*{e2}")
						}
					} else if *op == Token::Pow {
						if Expression::Float(Decimal::ZERO) == *e2.to_owned() {
							String::from("1")
						} else if Expression::Float(Decimal::ONE) == *e2.to_owned() {
							format!("{e1}")
						} else {
							format!("{e1}{op}{e2}")
						}
					} else {
						let s1 = format!("{e1}");
						let s2 = format!("{e2}");

						if s1.is_empty() {
							s2
						} else if s2.is_empty() {
							s1
						} else {
							format!("{e1}{op}{e2}")
						}
					}
				}
				Expression::Branched(e1, e2, e3) => format!("if {e1} then {e2} else {e3} end"),
				Expression::Identifier(ident) => ident.to_string(),
				Expression::Float(n) => n.to_string(),
				Expression::Matrix(matrix) => {
					let mut highest_padding_required = 0;
					let mut whitespace_index_map = vec![];

					for i in 0..matrix[0].len() {
						let mut max_len = 0;
						for row in matrix {
							if row[i].to_string().len() > max_len {
								max_len = row[i].to_string().len();
							}
						}
						whitespace_index_map.push(max_len);
					}

					let rows = matrix
						.iter()
						.map(|c| {
							let row = c
								.iter()
								.enumerate()
								.map(|(i, m)| {
									let l = m.to_string();
									if l.len() < whitespace_index_map[i] {
										" ".repeat(whitespace_index_map[i] - l.len()) + &m.to_string()
									} else {
										l
									}
								})
								.collect::<Vec<String>>()
								.join(" ");

							if row.len() > highest_padding_required {
								highest_padding_required = row.len();
							}

							format!("│ {row} │")
						})
						.collect::<Vec<String>>();

					format!(
						"┌ {} ┐\n{}\n└ {} ┘",
						" ".repeat(highest_padding_required),
						rows.join("\n"),
						" ".repeat(highest_padding_required),
					)
				}
				Expression::FunctionCall(ident, args) => format!(
					"{ident}({})",
					args
						.iter()
						.map(|(f, _)| format!("{f}"))
						.collect::<Vec<_>>()
						.join(",")
				),
			}
		)
	}
}
