use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, full_palette::*};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::Error;
use crate::expr::Expression;
use crate::interpreter::{Function, InterpreterContext};
use crate::standardlibrary::operators::{add, div, mul, sub};
use crate::types::Data;

pub fn abs(a: &Data) -> Data {
	match a {
		Data::Number(a, b) => Data::Number((a * a + b * b).sqrt(), 0.0),
		Data::Matrix(..) => determinant(a),
		_ => unimplemented!(),
	}
}

pub fn round(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.round(), y.round()),
		_ => unimplemented!(),
	}
}

pub fn ceil(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.round(), y.round()),
		_ => unimplemented!(),
	}
}

pub fn floor(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.round(), y.round()),
		_ => unimplemented!(),
	}
}

pub fn ln(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number((x * x + y * y).sqrt().ln(), y.atan2(*x)),
		_ => unimplemented!(),
	}
}

pub fn log10(a: &Data) -> Data {
	match a {
		Data::Number(..) => div(&ln(a), &ln(&Data::Number(10.0, 0.0))),
		_ => unimplemented!(),
	}
}

pub fn log(a: &Data, b: &Data) -> Data {
	match a {
		Data::Number(..) => div(&ln(a), &ln(b)),
		_ => unimplemented!(),
	}
}

pub fn sin(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.sin() * y.cosh(), x.cos() * y.sinh()),
		_ => unimplemented!(),
	}
}

pub fn cos(a: &Data) -> Data {
	match a {
		Data::Number(x, y) => Data::Number(x.cos() * y.cosh(), -x.sin() * y.sinh()),
		_ => unimplemented!(),
	}
}

pub fn tan(a: &Data) -> Data {
	div(&sin(a), &cos(a))
}

pub fn sqrt(a: &Data) -> Data {
	match a {
		Data::Number(a, b) => {
			let r = (a * a + b * b).sqrt();

			let zr = ((a + r) * (a + r) + b * b).sqrt();

			Data::Number(r.sqrt() * (a + r) / zr, r.sqrt() * b / zr)
		}
		_ => unimplemented!(),
	}
}

pub fn nrt(a: &Data, b: &Data) -> Data {
	if let Data::Number(x, y) = a
		&& let Data::Number(r, _) = abs(a)
		&& let Data::Number(b, _) = b
	{
		let z = r.powf(1.0 / b);

		let theta = (y / x).atan() / b;
		return Data::Number(z * theta.cos(), z * theta.sin());
	}

	unimplemented!()
}

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
				let mut delta = Data::Number(0.0, 0.0);

				for (i, n) in matrix[0].iter().enumerate() {
					let mut minor_matrix = matrix.clone();

					minor_matrix.remove(0);

					for row in &mut minor_matrix {
						row.remove(i);
					}

					delta = add(
						&delta,
						&mul(
							&mul(n, &Data::Number([1.0, -1.0][i % 2], 0.0)),
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

pub fn adj(v: &Data) -> Data {
	match v {
		Data::Matrix(matrix) => {
			let cols = matrix.len();
			let mut adj_matrix: Vec<Vec<Data>> = vec![];

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for adj");
				}
				adj_matrix.push((0..cols).map(|_| Data::Number(0.0, 0.0)).collect());
			}

			for i in 0..matrix.len() {
				for (j, n) in matrix[i].iter().enumerate() {
					let mut minor_matrix = matrix.clone();

					minor_matrix.remove(i);

					for row in &mut minor_matrix {
						row.remove(j);
					}

					adj_matrix[j][i] = mul(
						&mul(n, &Data::Number([1.0, -1.0][(i + j) % 2], 0.0)),
						&determinant(&Data::Matrix(minor_matrix)),
					);
				}
			}

			Data::Matrix(adj_matrix)
		}
		_ => panic!("expected matrix for adj"),
	}
}

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

pub fn graph<'a>(
	f: &Expression,
	ctx: &'a mut InterpreterContext<'a>,
) -> Result<Data, Error> {
	if let Expression::Identifier(f) = f
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
			let x = x as f32 / 50.0;

			ctx.0.insert("x".to_string(), Data::Number(x, 0.0));

			let data = code.clone().evaluate(ctx, g.range.clone())?;

			values.push((
				x,
				match data {
					Data::Number(a, _) => a,
					_ => panic!("expected number for plotting"),
				},
			));
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

		return Ok(Data::Number(0.0, 0.0));
	}
	// TODO: error handle this
	panic!("expected indentifier")
}