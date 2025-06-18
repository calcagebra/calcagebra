use std::{cmp::Ordering, ops::Rem};

use crate::{standardlibrary::math::inverse, types::Number};

pub fn add(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int(a + b),
		(Number::Int(a), Number::Real(b)) | (Number::Real(b), Number::Int(a)) => {
			Number::Real(*a as f32 + b)
		}
		(Number::Real(a), Number::Real(b)) => Number::Real(a + b),
		(Number::Int(n), Number::Complex(a, b)) | (Number::Complex(a, b), Number::Int(n)) => {
			Number::Complex(a + (*n as f32), *b)
		}
		(Number::Real(n), Number::Complex(a, b)) | (Number::Complex(a, b), Number::Real(n)) => {
			Number::Complex(a + n, *b)
		}
		(Number::Complex(a, b), Number::Complex(c, d)) => Number::Complex(a + c, b + d),
		(Number::Matrix(a), Number::Matrix(b)) => {
			if a.len() != b.len() {
				panic!("number of rows in matrix are not same, required for addition")
			}

			let mut r = vec![];

			let mut col = vec![];

			for (i, numbers) in a.iter().enumerate() {
				for (j, number) in numbers.iter().enumerate() {
					col.push(add(number, &b[i][j]));
				}
				r.push(col.clone());
				col.clear();
			}

			Number::Matrix(r)
		}
		_ => unimplemented!(),
	}
}

pub fn sub(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int(a - b),
		(Number::Int(a), Number::Real(b)) => Number::Real(*a as f32 - b),
		(Number::Real(b), Number::Int(a)) => Number::Real(b - *a as f32),
		(Number::Real(a), Number::Real(b)) => Number::Real(a - b),
		(Number::Int(n), Number::Complex(a, b)) => Number::Complex(-a + (*n as f32), -b),
		(Number::Complex(a, b), Number::Int(n)) => Number::Complex(a - (*n as f32), *b),
		(Number::Real(n), Number::Complex(a, b)) => Number::Complex(-a + n, -b),
		(Number::Complex(a, b), Number::Real(n)) => Number::Complex(a - n, *b),
		(Number::Complex(a, b), Number::Complex(c, d)) => Number::Complex(a - c, b - d),
		(Number::Matrix(a), Number::Matrix(b)) => {
			if a.len() != b.len() {
				panic!("number of rows in matrix are not same, required for subtraction")
			}

			let mut r = vec![];

			let mut col = vec![];

			for (i, numbers) in a.iter().enumerate() {
				for (j, number) in numbers.iter().enumerate() {
					col.push(sub(number, &b[i][j]));
				}
				r.push(col.clone());
				col.clear();
			}

			Number::Matrix(r)
		}
		_ => unimplemented!(),
	}
}

pub fn mul(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int(a * b),
		(Number::Int(a), Number::Real(b)) | (Number::Real(b), Number::Int(a)) => {
			Number::Real(*a as f32 * b)
		}
		(Number::Real(a), Number::Real(b)) => Number::Real(a * b),
		(Number::Int(n), Number::Complex(a, b)) | (Number::Complex(a, b), Number::Int(n)) => {
			Number::Complex(a * (*n as f32), b * (*n as f32))
		}
		(Number::Real(n), Number::Complex(a, b)) | (Number::Complex(a, b), Number::Real(n)) => {
			Number::Complex(a * n, b * n)
		}
		(Number::Complex(a, b), Number::Complex(c, d)) => Number::Complex(a * c - b * d, a * d + b * c),
		(Number::Matrix(a), Number::Matrix(b)) => {
			// TODO: Check if matrices can be multipled

			let mut r = vec![];

			let mut col = vec![];

			for row in a {
				let mut c = 0;
				while b[0].len() != c {
					let mut sum = Number::Real(0.0);

					for (k, number) in row.iter().enumerate() {
						sum = add(&sum, &mul(number, &b[k][c]));
					}

					col.push(sum);
					c += 1;
				}
				r.push(col.clone());
				col.clear();
			}

			Number::Matrix(r)
		}
		_ => unimplemented!(),
	}
}

pub fn div(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int(a / b),
		(Number::Int(a), Number::Real(b)) => Number::Real(*a as f32 / b),
		(Number::Real(b), Number::Int(a)) => Number::Real(b / *a as f32),
		(Number::Real(a), Number::Real(b)) => Number::Real(a / b),
		(Number::Int(n), Number::Complex(a, b)) => Number::Complex(
			(*n as f32) * a / (a * a + b * b),
			-(*n as f32) * b / (a * a + b * b),
		),
		(Number::Complex(a, b), Number::Int(n)) => Number::Complex(a / (*n as f32), b / (*n as f32)),
		(Number::Real(n), Number::Complex(a, b)) => {
			Number::Complex(n * a / (a * a + b * b), -n * b / (a * a + b * b))
		}
		(Number::Complex(a, b), Number::Real(n)) => Number::Complex(a / n, b / n),
		(Number::Complex(a, b), Number::Complex(c, d)) => Number::Complex(
			(a * c + b * d) / (c * c + d * d),
			(b * c - a * d) / (c * c + d * d),
		),
		_ => unimplemented!(),
	}
}

pub fn pow(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => {
			// TODO: Handle negative errors
			Number::Int(a.pow((*b).try_into().unwrap()))
		}
		(Number::Int(a), Number::Real(b)) => Number::Real((*a as f32).powf(*b)),
		(Number::Real(a), Number::Int(b)) => Number::Real(a.powf(*b as f32)),
		(Number::Real(a), Number::Real(b)) => Number::Real(a.powf(*b)),
		(Number::Complex(a, b), Number::Int(n)) => {
			let modulus = (a * a + b * b).sqrt();

			let argument = (b / a).atan();

			Number::Complex(
				modulus.powf(*n as f32) * (*n as f32 * argument).cos(),
				modulus.powf(*n as f32) * (*n as f32 * argument).sin(),
			)
		}
		(Number::Matrix(matrix), Number::Int(n)) => match n.cmp(&0) {
			Ordering::Less => {
				let mut resultant_matrix = inverse(vec![lhd.clone()]);

				for _ in 0..*n {
					resultant_matrix = mul(&resultant_matrix, &resultant_matrix);
				}

				resultant_matrix
			}
			Ordering::Equal => {
				let cols = matrix.len();
				let mut identity_matrix = vec![];

				for (i, row) in matrix.iter().enumerate() {
					if row.len() != cols {
						panic!("matrix should be square for pow");
					}

					identity_matrix.push(
						(0..cols)
							.map(|j| Number::Real(if i == j { 1.0 } else { 0.0 }))
							.collect(),
					);
				}

				Number::Matrix(identity_matrix)
			}
			Ordering::Greater => {
				let mut resultant_matrix = lhd.clone();

				for _ in 0..*n {
					resultant_matrix = mul(&resultant_matrix, &resultant_matrix);
				}

				resultant_matrix
			}
		},
		_ => unimplemented!(),
	}
}

pub fn rem(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int(a.rem(b)),
		(Number::Int(a), Number::Real(b)) => Number::Real((*a as f32).rem(b)),
		(Number::Real(a), Number::Int(b)) => Number::Real((a).rem(*b as f32)),
		(Number::Real(a), Number::Real(b)) => Number::Real(a.rem(b)),
		_ => unimplemented!(),
	}
}

pub fn is_eq(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int((a == b) as i32),
		(Number::Real(a), Number::Real(b)) => Number::Int((a == b) as i32),
		(Number::Complex(a, b), Number::Complex(c, d)) => Number::Int((a == c && b == d) as i32),
		(Number::Matrix(a), Number::Matrix(b)) => Number::Int((a == b) as i32),
		_ => unimplemented!(),
	}
}

pub fn neq(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int((a != b) as i32),
		(Number::Real(a), Number::Real(b)) => Number::Int((a != b) as i32),
		(Number::Complex(a, b), Number::Complex(c, d)) => Number::Int((a != c && b != d) as i32),
		(Number::Matrix(a), Number::Matrix(b)) => Number::Int((a != b) as i32),
		_ => unimplemented!(),
	}
}

pub fn gt(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int((a > b) as i32),
		(Number::Real(a), Number::Real(b)) => Number::Int((a > b) as i32),
		_ => unimplemented!(),
	}
}

pub fn gteq(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int((a >= b) as i32),
		(Number::Real(a), Number::Real(b)) => Number::Int((a >= b) as i32),
		_ => unimplemented!(),
	}
}

pub fn lt(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int((a < b) as i32),
		(Number::Real(a), Number::Real(b)) => Number::Int((a < b) as i32),
		_ => unimplemented!(),
	}
}

pub fn lteq(lhd: &Number, rhd: &Number) -> Number {
	match (lhd, rhd) {
		(Number::Int(a), Number::Int(b)) => Number::Int((a <= b) as i32),
		(Number::Real(a), Number::Real(b)) => Number::Int((a <= b) as i32),
		_ => unimplemented!(),
	}
}
