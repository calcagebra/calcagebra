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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
	Assignment((String, Option<DataType>), Box<Expression>),
	Abs(Box<Expression>),
	Binary(Box<Expression>, Token, Box<Expression>),
	Branched(Box<Expression>, Box<Expression>, Box<Expression>),
	Identifier(String),
	Float(f32),
	Matrix(Vec<Vec<Expression>>),
	FunctionCall(String, Vec<Expression>),
	FunctionDeclaration(String, Vec<(String, DataType)>, DataType, Box<Expression>),
}

impl Expression {
	pub fn evaluate<'a>(
		self,
		mut ctx: &'a mut InterpreterContext<'a>,
	) -> (&'a mut InterpreterContext<'a>, Data) {
		match self {
			Expression::Assignment((name, numbertype), expr) => {
				let number;

				(ctx, number) = expr.evaluate(ctx);

				if numbertype.is_some() && number.ty() != numbertype.unwrap() {
					// TODO: proper errors
					panic!(
						"type mismatch found {} expected {}",
						number,
						numbertype.unwrap()
					)
				}

				ctx.0.insert(name, number.clone());

				(ctx, number)
			}
			Expression::FunctionDeclaration(name, items, number_type, expr) => {
				ctx.1.insert(
					name.to_owned(),
					Function::UserDefined(UserDefinedFunction {
						params: items,
						return_type: number_type,
						code: *expr,
					}),
				);

				(ctx, Data::FnPointer(name))
			}
			Expression::Abs(expression) => {
				let data;
				(ctx, data) = expression.evaluate(ctx);
				(ctx, math::abs(&data))
			}
			Expression::Binary(lhs, token, rhs) => {
				let lhd;
				let rhd;

				(ctx, lhd) = lhs.evaluate(ctx);
				(ctx, rhd) = rhs.evaluate(ctx);

				(
					ctx,
					match token {
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
					},
				)
			}
			Expression::Branched(condition, then, otherwise) => {
				let data;

				(ctx, data) = condition.evaluate(ctx);

				if let Data::Number(condition, _) = data {
					return if condition != 0.0 {
						then.evaluate(ctx)
					} else {
						otherwise.evaluate(ctx)
					};
				}

				panic!("expected number in condition for branch statement")
			}
			Expression::Identifier(name) => {
				// TODO: Error handling for when name does not
				let data = ctx.0.get(&name).unwrap().to_owned();
				(ctx, data)
			}
			Expression::Float(f) => (ctx, Data::Number(f, 0.0)),
			Expression::Matrix(matrix) => {
				let mut matrix_data = vec![];

				for row in matrix {
					let mut row_data = vec![];
					for element in row {
						let data;

						(ctx, data) = element.evaluate(ctx);

						row_data.push(data);
					}
					matrix_data.push(row_data);
				}

				(ctx, Data::Matrix(matrix_data))
			}
			Expression::FunctionCall(name, mut exprs) => {
				if ctx.1.contains_key(&name) {
					let f = ctx.1.get(&name).unwrap().clone();

					if let Function::UserDefined(g) = f {
						for (i, (arg, numbertype)) in g.params.iter().enumerate() {
							let r;

							(ctx, r) = exprs.remove(i).evaluate(ctx);

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
