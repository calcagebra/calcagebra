use std::fmt::Display;

use cranelift::prelude::{types, Type};

use crate::{standardlibrary, token::Token};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum AstType {
	Int,
	Float,
}

impl AstType {
	pub fn parse(ident: &str) -> Self {
		match ident.to_uppercase().as_str() {
			"Z" | "INT" | "INTEGER" => Self::Int,
			"R" | "FLOAT" => Self::Float,
			_ => unimplemented!(),
		}
	}

	pub fn resolve(&self) -> Type {
		match self {
			AstType::Int => types::I64,
			AstType::Float => types::F64,
		}
	}
}

impl Display for AstType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				AstType::Int => "int",
				AstType::Float => "float",
			}
		)
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum AstNode {
	Assignment((String, AstType), Expression),
	FunctionCall(String, Vec<Expression>),
	FunctionDeclaration(String, Vec<(String, AstType)>, AstType, Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
	Abs(Box<Expression>),
	Binary(Box<Expression>, Token, Box<Expression>),
	Branched(Box<Expression>, Box<Expression>, Box<Expression>),
	Identifier(String),
	Float(f64),
	Integer(i64),
	FunctionCall(String, Vec<Expression>),
}

impl Expression {
	pub fn infer_datatype(&self) -> Option<AstType> {
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
					(AstType::Int, AstType::Int) => AstType::Int,
					(AstType::Int, AstType::Float)
					| (AstType::Float, AstType::Int)
					| (AstType::Float, AstType::Float) => AstType::Float,
				})
			}
			Expression::Identifier(_) => None,
			Expression::Float(_) => Some(AstType::Float),
			Expression::Integer(_) => Some(AstType::Int),
			Expression::FunctionCall(ident, _) => {
				if standardlibrary::is_standard_function(ident) {
					Some(standardlibrary::internal_type_map(ident).1)
				} else {
					None
				}
			}
		}
	}
}
