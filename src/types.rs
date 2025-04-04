use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Number {
	Int(i32),
	Real(f32),
}

impl Number {
	pub fn r#type(&self) -> NumberType {
		match self {
			Number::Int(..) => NumberType::Int,
			Number::Real(..) => NumberType::Real,
		}
	}

	pub fn int(&self) -> i32 {
		match self {
			Number::Int(i) => *i,
			Number::Real(f) => *f as i32,
		}
	}

	pub fn real(&self) -> f32 {
		match self {
			Number::Int(i) => *i as f32,
			Number::Real(f) => *f,
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Number::Int(i) => i.to_string(),
				Number::Real(f) => f.to_string(),
			}
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum NumberType {
	Int,
	Real,
}

impl NumberType {
	pub fn parse(ident: &str) -> Self {
		match ident.to_uppercase().as_str() {
			"Z" | "INT" | "INTEGER" => Self::Int,
			"R" | "FLOAT" => Self::Real,
			_ => unimplemented!(),
		}
	}
}

impl Display for NumberType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				NumberType::Int => "int",
				NumberType::Real => "float",
			}
		)
	}
}
