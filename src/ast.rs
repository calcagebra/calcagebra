use crate::{standardlibrary, token::Token, types::NumberType};

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum AstNode {
	Import(String),
	Assignment((String, NumberType), Expression),
	FunctionCall(String, Vec<Expression>),
	FunctionDeclaration(String, Vec<(String, NumberType)>, NumberType, Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
	Abs(Box<Expression>),
	Binary(Box<Expression>, Token, Box<Expression>),
	Branched(Box<Expression>, Box<Expression>, Box<Expression>),
	Identifier(String),
	Real(f32),
	Integer(i32),
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
				})
			}
			Expression::Identifier(_) => None,
			Expression::Real(_) => Some(NumberType::Real),
			Expression::Integer(_) => Some(NumberType::Int),
			Expression::FunctionCall(ident, _) => {
				if standardlibrary::is_simple_standard_function(ident)
					|| standardlibrary::is_complex_standard_function(ident)
				{
					Some(standardlibrary::internal_type_map(ident).1)
				} else {
					None
				}
			}
		}
	}
}
