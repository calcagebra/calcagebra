use crate::ast::Expression;
use crate::interpreter::Interpreter;
use crate::types::{Number, NumberType};
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{Color, IntoFont, full_palette::*};
use std::io::{Write, stdin, stdout};
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn is_simple_standard_function(f: &str) -> bool {
	[
		"print", "read", "int", "real", "round", "ceil", "floor", "ln", "log10", "log", "sin", "cos",
		"tan", "sqrt", "cbrt", "nrt",
	]
	.contains(&f)
}

pub fn is_complex_standard_function(f: &str) -> bool {
	["graph"].contains(&f)
}

pub fn simple_call(f: &str, args: Vec<Number>) -> Number {
	match f {
		"print" => print(args),
		"read" => read(),
		"int" => int(args),
		"real" => real(args),
		"round" => round(args),
		"ceil" => ceil(args),
		"floor" => floor(args),
		"ln" => ln(args),
		"log10" => log10(args),
		"log" => log(args),
		"sin" => sin(args),
		"cos" => cos(args),
		"tan" => tan(args),
		"sqrt" => sqrt(args),
		"cbrt" => cbrt(args),
		"nrt" => nrt(args),
		_ => unreachable!(),
	}
}

pub fn complex_call(f: &str, args: Vec<Expression>, interpreter: &mut Interpreter) -> Number {
	match f {
		"graph" => graph(args, interpreter),
		_ => unreachable!(),
	}
}

// IO
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

// TYPES
pub fn int(a: Vec<Number>) -> Number {
	Number::Int(a[0].int())
}

pub fn real(a: Vec<Number>) -> Number {
	Number::Real(a[0].real())
}

// MATH
pub fn round(a: Vec<Number>) -> Number {
	match a[0].r#type() {
		NumberType::Int => Number::Int(a[0].real().round() as i32),
		NumberType::Real => Number::Real(a[0].real().round()),
		_ => unimplemented!()
	}
}

pub fn ceil(a: Vec<Number>) -> Number {
	match a[0].r#type() {
		NumberType::Int => Number::Int(a[0].real().ceil() as i32),
		NumberType::Real => Number::Real(a[0].real().ceil()),
		_ => unimplemented!()
	}
}

pub fn floor(a: Vec<Number>) -> Number {
	match a[0].r#type() {
		NumberType::Int => Number::Int(a[0].real().floor() as i32),
		NumberType::Real => Number::Real(a[0].real().floor()),
		_ => unimplemented!()
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
		Number::Int(..) |
		Number::Real(..) => Number::Real(a[0].real().sqrt()),
		Number::Complex(a, b) => {
			let r = (a*a + b*b).sqrt();

			let zr = ((a+r)*(a+r) + b*b).sqrt();

			Number::Complex(r.sqrt() * (a + r) / zr, r.sqrt() * b / zr)
		},
		_ => unimplemented!()
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
