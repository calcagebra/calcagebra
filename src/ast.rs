use cranelift::prelude::{types, Type};

use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum AstType {
	Int = 1,
	Float = 0,
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
