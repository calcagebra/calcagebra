use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, full_palette::*};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::{Decimal, MathematicalOps, dec, prelude::*};
use std::f32;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::Error;
use crate::interpreter::{Function, InterpreterContext, Variable};
use crate::standardlibrary::operators::{add, div, mul, sub};
use crate::types::Data;

#[inline(always)]
pub fn abs(a: &Data) -> Data {
	match a {
		Data::Number(a, b) => Data::new_real((a * a + b * b).sqrt().unwrap()),
		Data::Matrix(..) => determinant(a),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn round(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.round(), y.round()),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn ceil(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.round(), y.round()),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn floor(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.round(), y.round()),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn exp(a: &Data) -> Data {
	let x = a.to_real();

	Data::Number(
		x.exp_with_tolerance(Decimal::from_parts(2, 0, 0, false, 28)),
		Decimal::ZERO,
	)
}

#[inline(always)]
pub fn ln(a: &Data) -> Data {
	let x = a.to_real();
	let y = a.to_img();

	let t = atan2(&Data::new_real(x), &Data::new_real(y)).to_real();

	Data::Number((x * x + y * y).sqrt().unwrap().ln(), t)
}

#[inline(always)]
pub fn log10(a: &Data) -> Data {
	match a {
		Data::Number(..) => div(&ln(a), &ln(&Data::new_real(Decimal::TEN))),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn log(a: &Data, b: &Data) -> Data {
	match a {
		Data::Number(..) => div(&ln(a), &ln(b)),
		_ => unimplemented!(),
	}
}

#[inline(always)]
pub fn sin(a: &Data) -> Data {
	let x = a.to_real();
	let y = a.to_img();

	if y == Decimal::ZERO {
		return Data::new_real((dec!(3.0) * Decimal::PI).sin());
	}

	let p = &mul(&Data::new_real(x.sin()), &cosh(&Data::new_real(y)));
	let q = &mul(&Data::new_real(x.cos()), &sinh(&Data::new_real(y)));

	Data::Number(p.to_real(), q.to_real())
}

#[inline(always)]
pub fn sinh(a: &Data) -> Data {
	div(
		&sub(
			&exp(a),
			&exp(&mul(a, &Data::new_real(Decimal::NEGATIVE_ONE))),
		),
		&Data::new_real(Decimal::TWO),
	)
}

#[inline(always)]
pub fn cos(a: &Data) -> Data {
	let x = a.to_real();
	let y = a.to_img();

	if y == Decimal::ZERO {
		return Data::new_real(x.cos());
	}

	let p = &mul(&Data::new_real(x.cos()), &cosh(&Data::new_real(y)));
	let q = &mul(&Data::new_real(-x.sin()), &sinh(&Data::new_real(y)));

	Data::Number(p.to_real(), q.to_real())
}

#[inline(always)]
pub fn cosh(a: &Data) -> Data {
	div(
		&add(
			&exp(a),
			&exp(&mul(a, &Data::new_real(Decimal::NEGATIVE_ONE))),
		),
		&Data::new_real(Decimal::TWO),
	)
}

#[inline(always)]
pub fn tan(a: &Data) -> Data {
	div(&sin(a), &cos(a))
}

/* atan, atan2 and i macro implementations from libm (rust) */
/* origin: FreeBSD /usr/src/lib/msun/src/e_atan2.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 *
 */

macro_rules! i {
	($array:expr, $index:expr) => {
		*$array.get($index).unwrap()
	};
	($array:expr, $index:expr, = , $rhs:expr) => {
		*$array.get_mut($index).unwrap() = $rhs;
	};
	($array:expr, $index:expr, -= , $rhs:expr) => {
		*$array.get_mut($index).unwrap() -= $rhs;
	};
	($array:expr, $index:expr, += , $rhs:expr) => {
		*$array.get_mut($index).unwrap() += $rhs;
	};
	($array:expr, $index:expr, &= , $rhs:expr) => {
		*$array.get_mut($index).unwrap() &= $rhs;
	};
	($array:expr, $index:expr, == , $rhs:expr) => {
		*$array.get_mut($index).unwrap() == $rhs
	};
}

#[inline(always)]
pub fn atan2(x: &Data, y: &Data) -> Data {
	let Data::Number(x, _) = x else {
		unimplemented!()
	};

	let Data::Number(y, _) = y else {
		unimplemented!()
	};

	let x = *x;
	let y = *y;

	const PI_LO: Decimal = dec!(1.224646799147E-16);

	let mut ix = (x.to_f64().unwrap().to_bits() >> 32) as u32;

	let lx = x.to_f64().unwrap().to_bits() as u32;

	let mut iy = (y.to_f64().unwrap().to_bits() >> 32) as u32;

	let ly = y.to_f64().unwrap().to_bits() as u32;

	if ((ix.wrapping_sub(0x3ff00000)) | lx) == 0 {
		/* x = 1.0 */

		return atan(&Data::Number(y, Decimal::ZERO));
	}

	let m = ((iy >> 31) & 1) | ((ix >> 30) & 2); /* 2*sign(x)+sign(y) */

	ix &= 0x7fffffff;

	iy &= 0x7fffffff;

	/* when y = 0 */

	if (iy | ly) == 0 {
		return Data::Number(
			match m {
				0 | 1 => y, /* atan(+-0,+anything)=+-0 */

				2 => Decimal::PI, /* atan(+0,-anything) = Decimal::PI */

				_ => -Decimal::PI, /* atan(-0,-anything) =-Decimal::PI */
			},
			Decimal::ZERO,
		);
	}

	/* when x = 0 */

	if (ix | lx) == 0 {
		return Data::Number(
			if m & 1 != 0 {
				-Decimal::HALF_PI
			} else {
				Decimal::HALF_PI
			},
			Decimal::ZERO,
		);
	}

	/* when x is INF */

	if ix == 0x7ff00000 {
		if iy == 0x7ff00000 {
			return Data::Number(
				match m {
					0 => Decimal::QUARTER_PI, /* atan(+INF,+INF) */

					1 => -Decimal::QUARTER_PI, /* atan(-INF,+INF) */

					2 => dec!(3.0) * Decimal::QUARTER_PI, /* atan(+INF,-INF) */

					_ => dec!(3.0) * Decimal::QUARTER_PI, /* atan(-INF,-INF) */
				},
				Decimal::ZERO,
			);
		} else {
			return Data::Number(
				match m {
					0 => Decimal::ZERO, /* atan(+...,+INF) */

					1 => -Decimal::ZERO, /* atan(-...,+INF) */

					2 => Decimal::PI, /* atan(+...,-INF) */

					_ => -Decimal::PI, /* atan(-...,-INF) */
				},
				Decimal::ZERO,
			);
		}
	}

	/* |y/x| > 0x1p64 */

	if ix.wrapping_add(64 << 20) < iy || iy == 0x7ff00000 {
		return Data::Number(
			if m & 1 != 0 {
				-Decimal::HALF_PI
			} else {
				Decimal::HALF_PI
			},
			Decimal::ZERO,
		);
	}

	/* z = atan(|y/x|) without spurious underflow */

	let z = if (m & 2 != 0) && iy.wrapping_add(64 << 20) < ix {
		/* |y/x| < 0x1p-64, x<0 */

		Decimal::ZERO
	} else {
		let Data::Number(t, _) = atan(&Data::Number((y / x).abs(), Decimal::ZERO)) else {
			unimplemented!()
		};

		t
	};

	Data::Number(
		match m {
			0 => z, /* atan(+,+) */

			1 => -z, /* atan(-,+) */

			2 => Decimal::PI - (z - PI_LO), /* atan(+,-) */

			_ => (z - PI_LO) - Decimal::PI, /* atan(-,-) */
		},
		Decimal::ZERO,
	)
}

#[inline(always)]
pub fn atan(x: &Data) -> Data {
	let Data::Number(x, _) = x else {
		unimplemented!()
	};

	let mut x = *x;

	const ATANHI: [Decimal; 4] = [
		dec!(4.63647609000806093515e-01), /* atan(0.5)hi 0x3FDDAC67, 0x0561BB4F */
		dec!(7.85398163397448278999e-01), /* atan(1.0)hi 0x3FE921FB, 0x54442D18 */
		dec!(9.82793723247329054082e-01), /* atan(1.5)hi 0x3FEF730B, 0xD281F69B */
		dec!(1.57079632679489655800e+00), /* atan(inf)hi 0x3FF921FB, 0x54442D18 */
	];

	const ATANLO: [Decimal; 4] = [
		dec!(2.26987774529e-17), /* atan(0.5)lo 0x3C7A2B7F, 0x222F65E2 */
		dec!(3.06161699786e-17), /* atan(1.0)lo 0x3C81A626, 0x33145C07 */
		dec!(1.39033110312e-17), /* atan(1.5)lo 0x3C700788, 0x7AF0CBBD */
		dec!(6.12323399573e-17), /* atan(inf)lo 0x3C91A626, 0x33145C07 */
	];

	const AT: [Decimal; 11] = [
		dec!(3.33333333333329318027e-01),  /* 0x3FD55555, 0x5555550D */
		dec!(-1.99999999998764832476e-01), /* 0xBFC99999, 0x9998EBC4 */
		dec!(1.42857142725034663711e-01),  /* 0x3FC24924, 0x920083FF */
		dec!(-1.11111104054623557880e-01), /* 0xBFBC71C6, 0xFE231671 */
		dec!(9.09088713343650656196e-02),  /* 0x3FB745CD, 0xC54C206E */
		dec!(-7.69187620504482999495e-02), /* 0xBFB3B0F2, 0xAF749A6D */
		dec!(6.66107313738753120669e-02),  /* 0x3FB10D66, 0xA0D03D51 */
		dec!(-5.83357013379057348645e-02), /* 0xBFADDE2D, 0x52DEFD9A */
		dec!(4.97687799461593236017e-02),  /* 0x3FA97B4B, 0x24760DEB */
		dec!(-3.65315727442169155270e-02), /* 0xBFA2B444, 0x2C6A6C2F */
		dec!(1.62858201153657823623e-02),  /* 0x3F90AD3A, 0xE322DA11 */
	];

	let mut ix = (x.to_f64().unwrap().to_bits() >> 32) as u32;

	let sign = ix >> 31;

	ix &= 0x7fff_ffff;

	if ix >= 0x4410_0000 {
		let z = ATANHI[3] + Decimal::from_f64(f64::from_bits(0x0380_0000)).unwrap(); // 0x1p-120f

		return Data::Number(if sign != 0 { -z } else { z }, Decimal::ZERO);
	}

	let id = if ix < 0x3fdc_0000 {
		/* |x| < 0.4375 */

		if ix < 0x3e40_0000 {
			/* |x| < 2^-27 */

			return Data::Number(x, Decimal::ZERO);
		}

		-1
	} else {
		x = x.abs();

		if ix < 0x3ff30000 {
			/* |x| < 1.1875 */

			if ix < 0x3fe60000 {
				/* 7/16 <= |x| < 11/16 */

				x = (Decimal::TWO * x - Decimal::ONE) / (Decimal::TWO + x);

				0
			} else {
				/* 11/16 <= |x| < 19/16 */

				x = (x - Decimal::ONE) / (x + Decimal::ONE);

				1
			}
		} else if ix < 0x40038000 {
			/* |x| < 2.4375 */

			x = (x - dec!(1.5)) / (Decimal::ONE + dec!(1.5) * x);

			2
		} else {
			/* 2.4375 <= |x| < 2^66 */

			x = -Decimal::ONE / x;

			3
		}
	};

	let z = x * x;

	let w = z * z;

	/* break sum from i=0 to 10 AT[i]z**(i+1) into odd and even poly */

	let s1 = z * (AT[0] + w * (AT[2] + w * (AT[4] + w * (AT[6] + w * (AT[8] + w * AT[10])))));

	let s2 = w * (AT[1] + w * (AT[3] + w * (AT[5] + w * (AT[7] + w * AT[9]))));

	if id < 0 {
		return Data::Number(x - x * (s1 + s2), Decimal::ZERO);
	}

	let z = i!(ATANHI, id as usize) - (x * (s1 + s2) - i!(ATANLO, id as usize) - x);

	Data::Number(if sign != 0 { -z } else { z }, Decimal::ZERO)
}

#[inline(always)]
pub fn sqrt(a: &Data) -> Data {
	let b = a.to_img();
	let a = a.to_real();

	let r = (a * a + b * b).sqrt().unwrap();

	let zr = ((a + r) * (a + r) + b * b).sqrt().unwrap();

	Data::Number(r.sqrt().unwrap() * (a + r) / zr, r.sqrt().unwrap() * b / zr)
}

#[inline(always)]
pub fn nrt(a: &Data, b: &Data) -> Data {
	if let Data::Number(x, y) = a
		&& let Data::Number(r, _) = abs(a)
		&& let Data::Number(b, _) = b
	{
		let z = r.powd(Decimal::ONE / b);

		let Data::Number(theta, _) = atan2(
			&Data::Number(*x, Decimal::ZERO),
			&Data::Number(*y, Decimal::ZERO),
		) else {
			unreachable!()
		};

		let theta = theta / b;

		return Data::Number(z * theta.cos(), z * theta.sin());
	}

	unimplemented!()
}

#[inline(always)]
pub fn determinant(v: &Data) -> Data {
	match v {
		Data::Matrix(matrix) => {
			let cols = matrix.len();

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for determinant");
				}
			}

			if cols == 2 && matrix[0].len() == 2 && matrix[1].len() == 2 {
				sub(
					&mul(&matrix[0][0], &matrix[1][1]),
					&mul(&matrix[0][1], &matrix[1][0]),
				)
			} else {
				let mut delta = Data::new_zero();

				for (i, n) in matrix[0].iter().enumerate() {
					let mut minor_matrix = matrix.clone();

					minor_matrix.remove(0);

					for row in &mut minor_matrix {
						row.remove(i);
					}

					delta = add(
						&delta,
						&mul(
							&mul(
								n,
								&Data::new_real([Decimal::ONE, Decimal::NEGATIVE_ONE][i % 2]),
							),
							&determinant(&Data::Matrix(minor_matrix)),
						),
					);
				}

				delta
			}
		}
		_ => panic!("expected matrix for determinant"),
	}
}

#[inline(always)]
pub fn transpose(v: &Data) -> Data {
	match v {
		Data::Matrix(matrix) => {
			let cols = matrix.len();

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for determinant");
				}
			}

			let mut iters: Vec<_> = matrix.iter().map(|n| n.iter()).collect();

			Data::Matrix(
				(0..cols)
					.map(|_| {
						iters
							.iter_mut()
							.map(|n| n.next().unwrap().clone())
							.collect::<Vec<Data>>()
					})
					.collect(),
			)
		}
		_ => panic!("expected matrix for transposing"),
	}
}

#[inline(always)]
pub fn adj(v: &Data) -> Data {
	match v {
		Data::Matrix(matrix) => {
			let cols = matrix.len();
			let mut adj_matrix: Vec<Vec<Data>> = vec![];

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for adj");
				}
				adj_matrix.push((0..cols).map(|_| Data::new_zero()).collect());
			}

			for i in 0..matrix.len() {
				for (j, n) in matrix[i].iter().enumerate() {
					let mut minor_matrix = matrix.clone();

					minor_matrix.remove(i);

					for row in &mut minor_matrix {
						row.remove(j);
					}

					adj_matrix[j][i] = mul(
						&mul(
							n,
							&Data::new_real([Decimal::ONE, Decimal::NEGATIVE_ONE][i % 2]),
						),
						&determinant(&Data::Matrix(minor_matrix)),
					);
				}
			}

			Data::Matrix(adj_matrix)
		}
		_ => panic!("expected matrix for adj"),
	}
}

#[inline(always)]
pub fn inverse(v: &Data) -> Data {
	match v {
		t @ Data::Matrix(matrix) => {
			let cols = matrix.len();

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for inverse");
				}
			}
			let det = &determinant(t);
			div(t, det)
		}
		_ => panic!("expected matrix for inverse"),
	}
}

#[inline(always)]
pub fn graph<'a, 'b>(f: &Data, ctx: &'a mut InterpreterContext<'b>) -> Result<Data, Error>
where
	'b: 'a,
{
	if let Data::Ident(f) = f
		&& let Function::UserDefined(g) = ctx.1.get(f).unwrap().clone()
	{
		let start = SystemTime::now();
		let duration = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
		let name = format!("graph-output-{duration}.png");

		let root = BitMapBackend::new(&name, (640, 480)).into_drawing_area();

		root.fill(&WHITE).unwrap();

		let mut chart = ChartBuilder::on(&root)
			.caption("Graph output", ("sans-serif", 20).into_font())
			.margin(5)
			.x_label_area_size(30)
			.y_label_area_size(30)
			.build_cartesian_2d(-10f32..10f32, -10f32..10f32)
			.unwrap();

		chart.configure_mesh().draw().unwrap();

		let style = &GREY_A700;
		let code = &g.code;

		let mut values = vec![];

		for x in -500..=500 {
			let x = x as f64 / 50.0;

			ctx.0.insert(
				"x".to_string(),
				Variable::new(Data::new_real(Decimal::from_f64(x).unwrap()), false),
			);

			let data = match code.clone().evaluate(ctx, g.range.clone()) {
				Ok(data) => match data {
					Data::Number(a, _) => a.to_f32().unwrap(),
					_ => {
						return Err(Error::LogicError(
							"expected number for plotting".to_string(),
						));
					}
				},
				Err(..) => f32::NAN,
			};

			values.push((x as f32, data));
		}

		chart
			.draw_series(LineSeries::new(values, &style))
			.unwrap()
			.label("Function")
			.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], *style));

		chart
			.draw_series(LineSeries::new((-10..=10).map(|x| (x as f32, 0.0)), &BLACK))
			.unwrap();

		chart
			.draw_series(LineSeries::new((-10..=10).map(|x| (0.0, x as f32)), &BLACK))
			.unwrap();

		chart
			.configure_series_labels()
			.background_style(WHITE.mix(0.8))
			.border_style(BLACK)
			.draw()
			.unwrap();

		root.present().unwrap();

		return Ok(Data::new_zero());
	}
	// TODO: error handle this
	panic!("expected indentifier")
}

#[inline(always)]
pub fn sum<'a, 'b>(
	f: &Data,
	a: &Data,
	b: &Data,
	ctx: &'a mut InterpreterContext<'b>,
) -> Result<Data, Error>
where
	'b: 'a,
{
	let Data::Ident(g) = f else { unreachable!() };

	let func = ctx.1.get(g).unwrap().clone();

	let mut sum = Data::new_zero();

	let a = a.to_real();
	let b = b.to_real();

	for i in a.to_i64().unwrap()..=b.to_i64().unwrap() {
		sum = add(
			&sum,
			&func.execute(ctx, vec![Data::new_real(Decimal::from_i64(i).unwrap())])?,
		)
	}

	Ok(sum)
}

#[inline(always)]
pub fn prod<'a, 'b>(
	f: &Data,
	a: &Data,
	b: &Data,
	ctx: &'a mut InterpreterContext<'b>,
) -> Result<Data, Error>
where
	'b: 'a,
{
	let Data::Ident(g) = f else { unreachable!() };

	let func = ctx.1.get(g).unwrap().clone();

	let mut prod = Data::new_real(Decimal::ONE);

	let a = a.to_real();
	let b = b.to_real();

	for i in a.to_i64().unwrap()..=b.to_i64().unwrap() {
		prod = mul(
			&prod,
			&func.execute(ctx, vec![Data::new_real(Decimal::from_i64(i).unwrap())])?,
		)
	}

	Ok(prod)
}

#[inline(always)]
pub fn differentiate<'a, 'b>(
	f: &Data,
	a: &Data,
	ctx: &'a mut InterpreterContext<'b>,
) -> Result<Data, Error>
where
	'b: 'a,
{
	let Data::Ident(g) = f else { unreachable!() };

	ctx.1.get(g).unwrap().clone().differentiate(a, &[], ctx)
}
