use std::{cmp::Ordering, ops::Rem};

use crate::{standardlibrary::math::inverse, types::Data};

pub fn add(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => Data::Number(a + c, b + d),
		(Data::Matrix(a), Data::Matrix(b)) => {
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

			Data::Matrix(r)
		}
		_ => unimplemented!(),
	}
}

pub fn sub(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => Data::Number(a - c, b - d),
		(Data::Matrix(a), Data::Matrix(b)) => {
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

			Data::Matrix(r)
		}
		_ => unimplemented!(),
	}
}

pub fn mul(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => Data::Number(a * c - b * d, a * d + b * c),
		(t @ Data::Number(..), Data::Matrix(b)) | (Data::Matrix(b), t @ Data::Number(..)) => {
			Data::Matrix(
				b.iter()
					.map(|c| c.iter().map(|d| mul(t, d)).collect())
					.collect(),
			)
		}
		(Data::Matrix(a), Data::Matrix(b)) => {
			// TODO: Check if matrices can be multipled

			let mut r = vec![];

			let mut col = vec![];

			for row in a {
				let mut c = 0;
				while b[0].len() != c {
					let mut sum = Data::Number(0.0, 0.0);

					for (k, number) in row.iter().enumerate() {
						sum = add(&sum, &mul(number, &b[k][c]));
					}

					col.push(sum);
					c += 1;
				}
				r.push(col.clone());
				col.clear();
			}

			Data::Matrix(r)
		}
	}
}

pub fn div(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => Data::Number(
			(a * c + b * d) / (c * c + d * d),
			(b * c - a * d) / (c * c + d * d),
		),
		(Data::Matrix(b), t @ Data::Number(..)) => Data::Matrix(
			b.iter()
				.map(|c| c.iter().map(|d| div(d, t)).collect())
				.collect(),
		),
		_ => unimplemented!(),
	}
}

pub fn pow(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(n, m)) => {
			if *m != 0.0 {
				unimplemented!("raising to complex number powers is not supported yet")
			}

			let modulus = (a * a + b * b).sqrt();

			let argument = (b / a).atan();

			Data::Number(
				modulus.powf(*n) * (*n * argument).cos(),
				modulus.powf(*n) * (*n * argument).sin(),
			)
		}
		(Data::Matrix(matrix), Data::Number(n, m)) => {
			if *m != 0.0 {
				unimplemented!("raising to complex number powers is not supported yet")
			}
			match (*n as i32).cmp(&0) {
				Ordering::Less => {
					let mut resultant_matrix = inverse(lhd);

					for _ in 0..(*n as i32) {
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
								.map(|j| Data::Number(if i == j { 1.0 } else { 0.0 }, 0.0))
								.collect(),
						);
					}

					Data::Matrix(identity_matrix)
				}
				Ordering::Greater => {
					let mut resultant_matrix = lhd.clone();

					for _ in 0..(*n as i32) {
						resultant_matrix = mul(&resultant_matrix, &resultant_matrix);
					}

					resultant_matrix
				}
			}
		}
		_ => unimplemented!(),
	}
}

pub fn rem(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != 0.0 || *d != 0.0 {
				unimplemented!("remainder of complex number powers is not supported yet")
			}
			Data::Number(a.rem(c), 0.0)
		}
		_ => unimplemented!(),
	}
}

pub fn is_eq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => Data::Number((a == c && b == d) as i32 as f32, 0.0),
		(Data::Matrix(a), Data::Matrix(b)) => Data::Number((a == b) as i32 as f32, 0.0),
		_ => unimplemented!(),
	}
}

pub fn neq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => Data::Number((a != c && b != d) as i32 as f32, 0.0),
		(Data::Matrix(a), Data::Matrix(b)) => Data::Number((a != b) as i32 as f32, 0.0),
		_ => unimplemented!(),
	}
}

pub fn gt(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != 0.0 || *d != 0.0 {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::Number((a > c) as i32 as f32, 0.0)
		}
		_ => unimplemented!(),
	}
}

pub fn gteq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != 0.0 || *d != 0.0 {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::Number((a >= c) as i32 as f32, 0.0)
		}
		_ => unimplemented!(),
	}
}

pub fn lt(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != 0.0 || *d != 0.0 {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::Number((a < c) as i32 as f32, 0.0)
		}
		_ => unimplemented!(),
	}
}

pub fn lteq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != 0.0 || *d != 0.0 {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::Number((a <= c) as i32 as f32, 0.0)
		}
		_ => unimplemented!(),
	}
}
