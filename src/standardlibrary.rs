use crate::ast::AstType;
use core::mem;
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{full_palette::*, Color, IntoFont};
use std::io::{stdin, stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn internal_type_map(f: &str) -> (Vec<AstType>, AstType) {
	match f {
		"read" => (vec![], AstType::Float),
		"float" => (vec![AstType::Int], AstType::Float),
		"int" => (vec![AstType::Float], AstType::Int),
		"print" | "round" | "ceil" | "floor" | "ln" | "log10" | "sin" | "cos" | "tan" | "sqrt"
		| "cbrt" | "graph" => (vec![AstType::Float], AstType::Float),
		"log" | "nrt" | "pow" => (vec![AstType::Float, AstType::Float], AstType::Float),
		_ => unimplemented!("type map not implemented for: {f}"),
	}
}

pub fn is_standard_function(f: &str) -> bool {
	[
		"read", "float", "int", "print", "round", "ceil", "floor", "ln", "log10", "sin", "cos", "tan",
		"sqrt", "cbrt", "graph", "log", "nrt", "pow",
	]
	.contains(&f)
}

// IO
pub fn print(a: f64) -> f64 {
	println!("{}", a);
	0.0
}

pub fn read() -> f64 {
	print!("Enter number: ");
	stdout().flush().unwrap();
	let mut buf = String::new();

	stdin().read_line(&mut buf).unwrap();

	buf.trim_end().parse::<f64>().unwrap()
}

// TYPES
pub fn int(a: f64) -> i64 {
	a as i64
}

pub fn float(a: i64) -> f64 {
	a as f64
}

// MATH
pub fn round(a: f64) -> f64 {
	a.round()
}

pub fn ceil(a: f64) -> f64 {
	a.ceil()
}

pub fn floor(a: f64) -> f64 {
	a.floor()
}

pub fn ln(a: f64) -> f64 {
	a.ln()
}

pub fn log10(a: f64) -> f64 {
	a.log10()
}

pub fn log(a: f64, b: f64) -> f64 {
	a.log(b)
}

pub fn sin(a: f64) -> f64 {
	a.sin()
}

pub fn cos(a: f64) -> f64 {
	a.cos()
}

pub fn tan(a: f64) -> f64 {
	a.tan()
}

pub fn sqrt(a: f64) -> f64 {
	a.sqrt()
}

pub fn cbrt(a: f64) -> f64 {
	a.cbrt()
}

pub fn nrt(a: f64, b: f64) -> f64 {
	a.powf(1.0 / b)
}

pub fn pow(a: f64, b: f64) -> f64 {
	a.powf(b)
}

pub fn graph(f: f64) -> f64 {
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

	chart
		.draw_series(LineSeries::new(
			(-500..=500).map(|x| x as f32 / 50.0).map(|x| {
				(x, unsafe {
					mem::transmute::<*const u8, fn(f64) -> f64>(f as u64 as *const u8)(x.into()) as f32
				})
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

	0.0
}
