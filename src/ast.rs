use crate::{standardlibrary, token::Token, types::DataType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum AstNode {
	Assignment((String, Option<DataType>), Expression),
	FunctionCall(String, Vec<Expression>),
	FunctionDeclaration(String, Vec<(String, DataType)>, DataType, Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
	Abs(Box<Expression>),
	Binary(Box<Expression>, Token, Box<Expression>),
	Branched(Box<Expression>, Box<Expression>, Box<Expression>),
	Identifier(String),
	Integer(i32),
	Real(f32),
	Matrix(Vec<Vec<Expression>>),
	FunctionCall(String, Vec<Expression>),
}

impl Expression {
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

				Some(match (lhs, rhs) {
					(DataType::Int, DataType::Int) => DataType::Int,
					(DataType::Int, DataType::Real)
					| (DataType::Real, DataType::Int)
					| (DataType::Real, DataType::Real) => DataType::Real,
					(DataType::Complex, _) | (_, DataType::Complex) => DataType::Complex,
					(DataType::Matrix, _) | (_, DataType::Matrix) => DataType::Matrix,
				})
			}
			Expression::Identifier(_) => None,
			Expression::Real(..) => Some(DataType::Real),
			Expression::Integer(..) => Some(DataType::Int),
			Expression::Matrix(..) => Some(DataType::Matrix),
			Expression::FunctionCall(ident, _) => {
				if standardlibrary::is_std(ident) {
					Some(standardlibrary::internal_type_map(ident).1)
				} else {
					None
				}
			}
		}
	}
}
