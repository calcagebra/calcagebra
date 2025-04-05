use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Number {
	Int(i32),
	Real(f32),
	Complex(f32, f32),
	Matrix(Vec<Vec<Number>>),
}

impl Number {
	pub fn r#type(&self) -> NumberType {
		match self {
			Number::Int(..) => NumberType::Int,
			Number::Real(..) => NumberType::Real,
			Number::Complex(..) => NumberType::Complex,
			Number::Matrix(..) => NumberType::Matrix,
		}
	}

	pub fn int(&self) -> i32 {
		match self {
			Number::Int(i) => *i,
			Number::Real(f) => *f as i32,
			_ => unimplemented!(),
		}
	}

	pub fn real(&self) -> f32 {
		match self {
			Number::Int(i) => *i as f32,
			Number::Real(f) => *f,
			_ => unimplemented!(),
		}
	}

	pub fn array(&self) -> Vec<f32> {
		match self {
			Number::Int(i) => vec![*i as f32],
			Number::Real(f) => vec![*f],
			Number::Complex(a, b) => vec![*a, *b],
			_ => unimplemented!(),
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
				Number::Complex(a, b) => format!("{a} + {b}i"),
				Number::Matrix(matrix) => {
					format!(
						"┌  {}  ┐\n{}\n└  {}  ┘",
						" ".repeat(matrix[0].len()),
						matrix
							.iter()
							.map(|c| format!(
								"│ {} │",
								c.iter()
									.map(|m| m.to_string())
									.collect::<Vec<String>>()
									.join(" ")
							))
							.collect::<Vec<String>>()
							.join("\n"),
						" ".repeat(matrix[0].len()),
					)
				}
			}
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum NumberType {
	Int,
	Real,
	Complex,
	Matrix,
}

impl NumberType {
	pub fn parse(ident: &str) -> Self {
		match ident.to_uppercase().as_str() {
			"Z" | "INT" | "INTEGER" => Self::Int,
			"R" | "FLOAT" => Self::Real,
			"C" | "COMPLEX" => Self::Complex,
			"MATRIX" => Self::Matrix,
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
				NumberType::Int => "Z",
				NumberType::Real => "R",
				NumberType::Complex => "C",
				NumberType::Matrix => "Matrix",
			}
		)
	}
}
