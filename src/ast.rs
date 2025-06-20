use crate::{standardlibrary, token::Token, types::NumberType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum AstNode {
	Assignment((String, Option<NumberType>), Expression),
	FunctionCall(String, Vec<Expression>),
	FunctionDeclaration(String, Vec<(String, NumberType)>, NumberType, Expression),
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
	pub fn infer_datatype(&self) -> Option<NumberType> {
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
					(NumberType::Int, NumberType::Int) => NumberType::Int,
					(NumberType::Int, NumberType::Real)
					| (NumberType::Real, NumberType::Int)
					| (NumberType::Real, NumberType::Real) => NumberType::Real,
					(NumberType::Complex, _) | (_, NumberType::Complex) => NumberType::Complex,
					(NumberType::Matrix, _) | (_, NumberType::Matrix) => NumberType::Matrix,
				})
			}
			Expression::Identifier(_) => None,
			Expression::Real(..) => Some(NumberType::Real),
			Expression::Integer(..) => Some(NumberType::Int),
			Expression::Matrix(..) => Some(NumberType::Matrix),
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
