use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
	Number(Decimal, Decimal),
	Matrix(Vec<Vec<Data>>),
	FnPointer(String),
}

impl Data {
	pub fn ty(&self) -> DataType {
		match self {
			Data::Number(..) => DataType::Number,
			Data::Matrix(..) => DataType::Matrix,
			Data::FnPointer(..) => DataType::FnPointer,
		}
	}
}

impl Display for Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Data::Number(a, b) => {
					if *b == Decimal::ZERO {
						format!("{a}")
					} else if *a == Decimal::ZERO {
						format!("{b}i")
					} else {
						format!("{a} + {b}i")
					}
				}
				Data::Matrix(matrix) => {
					let mut highest_padding_required = 0;
					let mut whitespace_index_map = vec![];

					for i in 0..matrix[0].len() {
						let mut max_len = 0;
						for row in matrix {
							if row[i].to_string().len() > max_len {
								max_len = row[i].to_string().len();
							}
						}
						whitespace_index_map.push(max_len);
					}

					let rows = matrix
						.iter()
						.map(|c| {
							let row = c
								.iter()
								.enumerate()
								.map(|(i, m)| {
									let l = m.to_string();
									if l.len() < whitespace_index_map[i] {
										" ".repeat(whitespace_index_map[i] - l.len()) + &m.to_string()
									} else {
										l
									}
								})
								.collect::<Vec<String>>()
								.join(" ");

							if row.len() > highest_padding_required {
								highest_padding_required = row.len();
							}

							format!("│ {row} │")
						})
						.collect::<Vec<String>>();

					format!(
						"┌ {} ┐\n{}\n└ {} ┘",
						" ".repeat(highest_padding_required),
						rows.join("\n"),
						" ".repeat(highest_padding_required),
					)
				}
				Data::FnPointer(str) => str.to_owned(),
			}
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum DataType {
	Number,
	Matrix,
	FnPointer,
}

impl DataType {
	pub fn parse(ident: &str) -> Self {
		match ident.to_uppercase().as_str() {
			"C" | "COMPLEX" => Self::Number,
			"M" | "MATRIX" => Self::Matrix,
			"FN" => Self::FnPointer,
			_ => unimplemented!(),
		}
	}
}

impl Display for DataType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				DataType::Number => "C",
				DataType::Matrix => "Matrix",
				DataType::FnPointer => "Fn",
			}
		)
	}
}
