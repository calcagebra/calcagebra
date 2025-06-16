use crate::ast::Expression;
use crate::interpreter::Interpreter;
use crate::types::{Number, NumberType};

pub fn internal_type_map(f: &str) -> (Vec<NumberType>, NumberType) {
	match f {
		"read" => (vec![], NumberType::Real),
		"real" => (vec![NumberType::Int], NumberType::Real),
		"int" => (vec![NumberType::Real], NumberType::Int),
		"print" | "round" | "ceil" | "floor" | "ln" | "log10" | "sin" | "cos" | "tan" | "sqrt"
		| "cbrt" | "graph" => (vec![NumberType::Real], NumberType::Real),
		"log" | "nrt" => (vec![NumberType::Real, NumberType::Real], NumberType::Real),
		_ => unimplemented!("type map not implemented for: {f}"),
	}
}

pub fn is_std(f: &str) -> bool {
	[
		"print", "read", "int", "real", "add", "sub", "mul", "div", "pow", "rem", "is_eq", "neq", "gt",
		"gteq", "lt", "lteq", "round", "ceil", "floor", "ln", "log10", "log", "sin", "cos", "tan",
		"sqrt", "cbrt", "nrt", "graph",
	]
	.contains(&f)
}

pub fn needs_ctx(f: &str) -> bool {
	["graph"].contains(&f)
}

pub fn call(f: &str, args: Vec<Number>) -> Number {
	match f {
		"print" => io::print(args),
		"read" => io::read(),
		"int" => types::int(args),
		"real" => types::real(args),
		"round" => math::round(args),
		"ceil" => math::ceil(args),
		"floor" => math::floor(args),
		"ln" => math::ln(args),
		"log10" => math::log10(args),
		"log" => math::log(args),
		"sin" => math::sin(args),
		"cos" => math::cos(args),
		"tan" => math::tan(args),
		"sqrt" => math::sqrt(args),
		"cbrt" => math::cbrt(args),
		"nrt" => math::nrt(args),
		_ => unreachable!(),
	}
}

pub fn ctx_call(f: &str, args: Vec<Expression>, interpreter: &mut Interpreter) -> Number {
	match f {
		"graph" => math::graph(args, interpreter),
		_ => unreachable!(),
	}
}

mod io {
	use std::io::{Write, stdin, stdout};

	use crate::types::Number;

	pub fn print(a: Vec<Number>) -> Number {
		for b in a {
			println!("{}", b);
		}
		Number::Real(0.0)
	}

	pub fn read() -> Number {
		print!("Enter number: ");
		stdout().flush().unwrap();
		let mut buf = String::new();

		stdin().read_line(&mut buf).unwrap();

		Number::Real(buf.trim_end().parse::<f32>().unwrap())
	}
}

mod types {
	use crate::types::Number;

	pub fn int(a: Vec<Number>) -> Number {
		Number::Int(a[0].int())
	}

	pub fn real(a: Vec<Number>) -> Number {
		Number::Real(a[0].real())
	}
}

pub mod operands {
	use std::ops::Rem;

	use crate::types::Number;

	pub fn add(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => return Number::Int(a + b),
			(Number::Int(a), Number::Real(b)) | (Number::Real(b), Number::Int(a)) => {
				return Number::Real(*a as f32 + b);
			}
			(Number::Real(a), Number::Real(b)) => return Number::Real(a + b),
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
						col.push(add(&number, &b[i][j]));
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
			(Number::Int(a), Number::Int(b)) => return Number::Int(a - b),
			(Number::Int(a), Number::Real(b)) => {
				return Number::Real(*a as f32 - b);
			}
			(Number::Real(b), Number::Int(a)) => {
				return Number::Real(b - *a as f32);
			}
			(Number::Real(a), Number::Real(b)) => return Number::Real(a - b),
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
			(Number::Int(a), Number::Int(b)) => return Number::Int(a * b),
			(Number::Int(a), Number::Real(b)) | (Number::Real(b), Number::Int(a)) => {
				return Number::Real(*a as f32 * b);
			}
			(Number::Real(a), Number::Real(b)) => return Number::Real(a * b),
			(Number::Int(n), Number::Complex(a, b)) | (Number::Complex(a, b), Number::Int(n)) => {
				Number::Complex(a * (*n as f32), b * (*n as f32))
			}
			(Number::Real(n), Number::Complex(a, b)) | (Number::Complex(a, b), Number::Real(n)) => {
				Number::Complex(a * n, b * n)
			}
			(Number::Complex(a, b), Number::Complex(c, d)) => {
				Number::Complex(a * c - b * d, a * d + b * c)
			}
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
			(Number::Int(a), Number::Int(b)) => return Number::Int(a / b),
			(Number::Int(a), Number::Real(b)) => {
				return Number::Real(*a as f32 / b);
			}
			(Number::Real(b), Number::Int(a)) => {
				return Number::Real(b / *a as f32);
			}
			(Number::Real(a), Number::Real(b)) => return Number::Real(a / b),
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
				return Number::Int(a.pow((*b).try_into().unwrap()));
			}
			(Number::Int(a), Number::Real(b)) => {
				return Number::Real((*a as f32).powf(*b));
			}
			(Number::Real(a), Number::Int(b)) => {
				return Number::Real(a.powf(*b as f32));
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Real(a.powf(*b));
			}
			(Number::Complex(a, b), Number::Int(n)) => {
				let modulus = (a * a + b * b).sqrt();

				let argument = (b / a).atan();

				return Number::Complex(
					modulus.powf(*n as f32) * (*n as f32 * argument).cos(),
					modulus.powf(*n as f32) * (*n as f32 * argument).sin(),
				);
			}
			_ => unimplemented!(),
		}
	}

	pub fn rem(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => return Number::Int(a.rem(b)),
			(Number::Int(a), Number::Real(b)) => {
				return Number::Real((*a as f32).rem(b));
			}
			(Number::Real(a), Number::Int(b)) => {
				return Number::Real((a).rem(*b as f32));
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Real(a.rem(b));
			}
			_ => unimplemented!(),
		}
	}

	pub fn is_eq(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => {
				return Number::Int((a == b) as i32);
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Int((a == b) as i32);
			}
			(Number::Complex(a, b), Number::Complex(c, d)) => {
				return Number::Int((a == c && b == d) as i32);
			}
			(Number::Matrix(a), Number::Matrix(b)) => {
				return Number::Int((a == b) as i32);
			}
			_ => unimplemented!(),
		}
	}

	pub fn neq(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => {
				return Number::Int((a != b) as i32);
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Int((a != b) as i32);
			}
			(Number::Complex(a, b), Number::Complex(c, d)) => {
				return Number::Int((a != c && b != d) as i32);
			}
			(Number::Matrix(a), Number::Matrix(b)) => {
				return Number::Int((a != b) as i32);
			}
			_ => unimplemented!(),
		}
	}

	pub fn gt(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => {
				return Number::Int((a > b) as i32);
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Int((a > b) as i32);
			}
			_ => unimplemented!(),
		}
	}

	pub fn gteq(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => {
				return Number::Int((a >= b) as i32);
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Int((a >= b) as i32);
			}
			_ => unimplemented!(),
		}
	}

	pub fn lt(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => {
				return Number::Int((a < b) as i32);
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Int((a < b) as i32);
			}
			_ => unimplemented!(),
		}
	}

	pub fn lteq(lhd: &Number, rhd: &Number) -> Number {
		match (lhd, rhd) {
			(Number::Int(a), Number::Int(b)) => {
				return Number::Int((a <= b) as i32);
			}
			(Number::Real(a), Number::Real(b)) => {
				return Number::Int((a <= b) as i32);
			}
			_ => unimplemented!(),
		}
	}
}

mod math {
	use plotters::backend::BitMapBackend;
	use plotters::chart::ChartBuilder;
	use plotters::drawing::IntoDrawingArea;
	use plotters::element::PathElement;
	use plotters::series::LineSeries;
	use plotters::style::{Color, IntoFont, full_palette::*};
	use std::time::{SystemTime, UNIX_EPOCH};

	use crate::ast::Expression;
	use crate::interpreter::Interpreter;
	use crate::types::{Number, NumberType};

	pub fn round(a: Vec<Number>) -> Number {
		match a[0].r#type() {
			NumberType::Int => Number::Int(a[0].real().round() as i32),
			NumberType::Real => Number::Real(a[0].real().round()),
			_ => unimplemented!(),
		}
	}

	pub fn ceil(a: Vec<Number>) -> Number {
		match a[0].r#type() {
			NumberType::Int => Number::Int(a[0].real().ceil() as i32),
			NumberType::Real => Number::Real(a[0].real().ceil()),
			_ => unimplemented!(),
		}
	}

	pub fn floor(a: Vec<Number>) -> Number {
		match a[0].r#type() {
			NumberType::Int => Number::Int(a[0].real().floor() as i32),
			NumberType::Real => Number::Real(a[0].real().floor()),
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
}
