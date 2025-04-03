use std::fmt::Display;


#[derive(Debug, Clone, Copy)]
pub enum Number {
	Bool(bool),
	Int(i64),
	Real(f64),
}

impl Number {
	pub fn r#type(&self) -> NumberType {
		match self {
			Number::Bool(..) => NumberType::Bool,
			Number::Int(..) => NumberType::Int,
			Number::Real(..) => NumberType::Real
		}
	}

	pub fn int(&self) -> i64 {
		match self {
			Number::Int(i) => *i,
			_ => unreachable!(),
		}
	}

	pub fn real(&self) -> f64 {
		match self {
			Number::Real(f) => *f,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum NumberType {
	Bool,
	Int,
	Real,
}

impl NumberType {
	pub fn parse(ident: &str) -> Self {
		match ident.to_uppercase().as_str() {
			"BOOL" => Self::Bool,
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
				NumberType::Bool => "bool",
				NumberType::Int => "int",
				NumberType::Real => "float",
			}
		)
	}
}
