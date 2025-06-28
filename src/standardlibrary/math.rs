use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, full_palette::*};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ast::Expression;
use crate::interpreter::Interpreter;
use crate::standardlibrary::operands::{add, div, mul, sub};
use crate::types::{Number, NumberType};

pub fn abs(a: Vec<Number>) -> Number {
	let numbertype = a[0].r#type();

	match numbertype {
		NumberType::Int => Number::Real(a[0].int().abs() as f32),
		NumberType::Real => Number::Real(a[0].real().abs()),
		NumberType::Complex => Number::Real(a[0].array().iter().map(|f| f * f).sum::<f32>().sqrt()),
		NumberType::Matrix => determinant(a),
	}
}

pub fn round(a: Vec<Number>) -> Number {
	match a[0].r#type() {
		NumberType::Real => Number::Int(a[0].real().round() as i32),
		_ => unimplemented!(),
	}
}

pub fn ceil(a: Vec<Number>) -> Number {
	match a[0].r#type() {
		NumberType::Real => Number::Int(a[0].real().ceil() as i32),
		_ => unimplemented!(),
	}
}

pub fn floor(a: Vec<Number>) -> Number {
	match a[0].r#type() {
		NumberType::Real => Number::Int(a[0].real().floor() as i32),
		_ => unimplemented!(),
	}
}

pub fn ln(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().ln())
}

pub fn log10(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().log10())
}

pub fn log(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().log(a[1].real()))
}

pub fn sin(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().sin())
}

pub fn cos(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().cos())
}

pub fn tan(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().tan())
}

pub fn sqrt(a: Vec<Number>) -> Number {
	match a[0] {
		Number::Int(..) | Number::Real(..) => Number::Real(a[0].real().sqrt()),
		Number::Complex(a, b) => {
			let r = (a * a + b * b).sqrt();

			let zr = ((a + r) * (a + r) + b * b).sqrt();

			Number::Complex(r.sqrt() * (a + r) / zr, r.sqrt() * b / zr)
		}
		_ => unimplemented!(),
	}
}

pub fn cbrt(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().cbrt())
}

pub fn nrt(a: Vec<Number>) -> Number {
	Number::Real(a[0].real().powf(1.0 / a[1].real()))
}

pub fn determinant(v: Vec<Number>) -> Number {
	match &v[0] {
		Number::Matrix(matrix) => {
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
				let mut delta = Number::Real(0.0);

				for (i, n) in matrix[0].iter().enumerate() {
					let mut minor_matrix = matrix.clone();

					minor_matrix.remove(0);

					for row in &mut minor_matrix {
						row.remove(i);
					}

					delta = add(
						&delta,
						&mul(
							&mul(n, &Number::Int([1, -1][i % 2])),
							&determinant(vec![Number::Matrix(minor_matrix)]),
						),
					);
				}

				delta
			}
		}
		_ => panic!("expected matrix for determinant"),
	}
}

pub fn transpose(v: Vec<Number>) -> Number {
	match &v[0] {
		Number::Matrix(matrix) => {
			let cols = matrix.len();

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for determinant");
				}
			}

			let mut iters: Vec<_> = matrix.iter().map(|n| n.iter()).collect();
			Number::Matrix(
				(0..cols)
					.map(|_| {
						iters
							.iter_mut()
							.map(|n| n.next().unwrap().clone())
							.collect::<Vec<Number>>()
					})
					.collect(),
			)
		}
		_ => panic!("expected matrix for transposing"),
	}
}

pub fn adj(v: Vec<Number>) -> Number {
	match &v[0] {
		Number::Matrix(matrix) => {
			let cols = matrix.len();
			let mut adj_matrix: Vec<Vec<Number>> = vec![];

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for adj");
				}
				adj_matrix.push((0..cols).map(|_| Number::Real(0.0)).collect());
			}

			for i in 0..matrix.len() {
				for (j, n) in matrix[i].iter().enumerate() {
					let mut minor_matrix = matrix.clone();

					minor_matrix.remove(i);

					for row in &mut minor_matrix {
						row.remove(j);
					}

					adj_matrix[j][i] = mul(
						&mul(n, &Number::Int([1, -1][(i + j) % 2])),
						&determinant(vec![Number::Matrix(minor_matrix)]),
					);
				}
			}

			Number::Matrix(adj_matrix)
		}
		_ => panic!("expected matrix for adj"),
	}
}

pub fn inverse(v: Vec<Number>) -> Number {
	match &v[0] {
		t @ Number::Matrix(matrix) => {
			let cols = matrix.len();

			for row in matrix {
				if row.len() != cols {
					panic!("matrix should be square for inverse");
				}
			}
			let det = &determinant(vec![t.clone()]);
			div(t, det)
		}
		_ => panic!("expected matrix for inverse"),
	}
}

pub fn graph(f: Vec<Expression>, interpreter: &mut Interpreter) -> Number {
	if let Expression::Identifier(f) = &f[0] {
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
		let code = interpreter.functions.get(f).unwrap().code.clone();

		chart
			.draw_series(LineSeries::new(
				(-500..=500).map(|x| x as f32 / 50.0).map(|x| {
					interpreter.globals.insert("x".to_string(), Number::Real(x));
					(x, interpreter.interpret_expression(&code).real())
				}),
				&style,
			))
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

		return Number::Real(0.0);
	}
	// TODO: error handle this
	panic!("expected indentifier")
}
