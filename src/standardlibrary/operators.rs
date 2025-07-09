use std::{cmp::Ordering, ops::Rem};

use rust_decimal::{Decimal, MathematicalOps, prelude::FromPrimitive};

use crate::{
	standardlibrary::math::{atan2, inverse},
	types::Data,
};

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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
					let mut sum = Data::new_zero();

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
		_ => unimplemented!(),
	}
}

#[inline(always)]
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

#[inline(always)]
pub fn pow(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(n, m)) => {
			if *m != Decimal::ZERO {
				unimplemented!("raising to complex number powers is not supported yet")
			}

			if *b == Decimal::ZERO {
				return Data::new_real(a.powd(*n));
			}

			let modulus = (a * a + b * b).sqrt().unwrap();

			let argument = atan2(&Data::new_real(*a), &Data::new_real(*b)).to_real();

			Data::Number(
				modulus.powd(*n) * (*n * argument).cos(),
				modulus.powd(*n) * (*n * argument).sin(),
			)
		}
		(Data::Matrix(matrix), Data::Number(n, m)) => {
			if *m != Decimal::ZERO {
				unimplemented!("raising to complex number powers is not supported yet")
			}
			match n.cmp(&Decimal::ZERO) {
				Ordering::Less => {
					let mut resultant_matrix = inverse(lhd);

					for _ in 0..n.to_string().parse::<i64>().unwrap() {
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
								.map(|j| {
									Data::Number(
										if i == j { Decimal::ONE } else { Decimal::ZERO },
										Decimal::ZERO,
									)
								})
								.collect(),
						);
					}

					Data::Matrix(identity_matrix)
				}
				Ordering::Greater => {
					let mut resultant_matrix = lhd.clone();

					for _ in 0..n.to_string().parse::<i64>().unwrap() {
						resultant_matrix = mul(&resultant_matrix, &resultant_matrix);
					}

					resultant_matrix
				}
			}
		}
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn rem(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != Decimal::ZERO || *d != Decimal::ZERO {
				unimplemented!("remainder of complex number powers is not supported yet")
			}
			Data::new_real(a.rem(c))
		}
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn is_eq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			Data::new_real(Decimal::from_u8((a == c && b == d) as u8).unwrap())
		}
		(Data::Matrix(a), Data::Matrix(b)) => Data::new_real(Decimal::from_u8((a == b) as u8).unwrap()),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn neq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			Data::new_real(Decimal::from_u8((a != c && b != d) as u8).unwrap())
		}
		(Data::Matrix(a), Data::Matrix(b)) => Data::new_real(Decimal::from_u8((a != b) as u8).unwrap()),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn gt(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != Decimal::ZERO || *d != Decimal::ZERO {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::new_real(Decimal::from_u8((a > c) as u8).unwrap())
		}
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn gteq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != Decimal::ZERO || *d != Decimal::ZERO {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::new_real(Decimal::from_u8((a >= c) as u8).unwrap())
		}
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn lt(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != Decimal::ZERO || *d != Decimal::ZERO {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::new_real(Decimal::from_u8((a < c) as u8).unwrap())
		}
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn lteq(lhd: &Data, rhd: &Data) -> Data {
	match (lhd, rhd) {
		(Data::Number(a, b), Data::Number(c, d)) => {
			if *b != Decimal::ZERO || *d != Decimal::ZERO {
				unimplemented!("relational operators on complex number powers is not supported yet")
			}
			Data::new_real(Decimal::from_u8((a <= c) as u8).unwrap())
		}
		_ => unimplemented!(),
	}
}
