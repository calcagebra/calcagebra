use std::ops::Range;

use crate::errors::{Error, TypeError};
use crate::interpreter::UserDefinedFunction;
use crate::standardlibrary::operators::{
	add, div, gt, gteq, is_eq, lt, lteq, mul, neq, pow, rem, sub,
};
use crate::standardlibrary::{self, math};
use crate::{
	interpreter::{Function, InterpreterContext},
	token::Token,
	types::{Data, DataType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	Assignment((String, Option<DataType>), Box<Expression>),
	Abs(Box<Expression>),
	Binary(Box<Expression>, Token, Box<Expression>),
	Branched(Box<Expression>, Box<Expression>, Box<Expression>),
	Identifier(String),
	Float(f64),
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

				ctx.0.insert(name, number.clone());

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

				Ok(Data::FnPointer(name))
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
					return if condition != 0.0 {
						Ok(then.evaluate(ctx, range.clone())?)
					} else {
						Ok(otherwise.evaluate(ctx, range.clone())?)
					};
				}

				Err(TypeError::new(DataType::Number, data.ty(), 0..0).to_error())
			}
			Expression::Identifier(name) => {
				if ctx.0.contains_key(&name) {
					Ok(ctx.0.get(&name).unwrap().to_owned())
				} else if ctx.1.contains_key(&name) {
					Ok(Data::FnPointer(name))
				} else {
					Err(Error::LogicError(format!("undefined variable: `{name}`")))
				}
			}
			Expression::Float(f) => Ok(Data::Number(f, 0.0)),
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
				if ctx.1.contains_key(&name) {
					let f = ctx.1.get(&name).unwrap().clone();

					if let Function::UserDefined(g) = f {
						let mut args = vec![];

						for (expr, range) in exprs {
							let data = expr.evaluate(ctx, range)?;
							args.push(data);
						}

						let data = g.execute(ctx, args);

						return data;
					} else if let Function::STD(g) = f {
						let mut args = vec![];

						for (expr, range) in exprs {
							let data = expr.evaluate(ctx, range)?;
							args.push(data);
						}

						return g.execute(ctx, args);
					}
				}

				Err(Error::LogicError(format!("undefined function: `{name}`")))
			}
		}
	}

	pub fn infer_datatype(&self) -> Option<DataType> {
		match self {
			Expression::Abs(expression) => expression.infer_datatype(),
			Expression::Branched(..) => None,
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
				if standardlibrary::is_std(ident) {
					Some(standardlibrary::internal_type_map(ident).1)
				} else {
					None
				}
			}
			Expression::Assignment(_, expression) => expression.infer_datatype(),
			Expression::FunctionDeclaration(..) => Some(DataType::FnPointer),
		}
	}
}
