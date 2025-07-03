use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
	Number(f32, f32),
	Matrix(Vec<Vec<Data>>),
}

impl Display for Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Data::Number(a, b) => format!("{a} + {b}i"),
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
			}
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum DataType {
	Number,
	Matrix,
}

impl DataType {
	pub fn parse(ident: &str) -> Self {
		match ident.to_uppercase().as_str() {
			"C" | "COMPLEX" => Self::Number,
			"M" | "MATRIX" => Self::Matrix,
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
			}
		)
	}
}
