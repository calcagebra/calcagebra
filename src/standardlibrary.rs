use core::mem;
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{full_palette::*, Color, IntoFont};
use std::io::{stdin, stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// IO
pub extern "C" fn print(value: f64) -> f64 {
	println!("{}", value);
	0.0
}

pub extern "C" fn read() -> f64 {
	print!("Enter number: ");
	stdout().flush().unwrap();
	let mut buf = String::new();

	stdin().read_line(&mut buf).unwrap();

	buf.trim_end().parse::<f64>().unwrap()
}

// MATH
pub extern "C" fn round(a: f64) -> f64 {
	a.round()
}

pub extern "C" fn ceil(a: f64) -> f64 {
	a.ceil()
}

pub extern "C" fn floor(a: f64) -> f64 {
	a.floor()
}

pub extern "C" fn ln(a: f64) -> f64 {
	a.ln()
}

pub extern "C" fn log10(a: f64) -> f64 {
	a.log10()
}

pub extern "C" fn log(a: f64, b: f64) -> f64 {
	a.log(b)
}

pub extern "C" fn sin(a: f64) -> f64 {
	a.sin()
}

pub extern "C" fn cos(a: f64) -> f64 {
	a.cos()
}

pub extern "C" fn tan(a: f64) -> f64 {
	a.tan()
}

pub extern "C" fn sqrt(a: f64) -> f64 {
	a.sqrt()
}

pub extern "C" fn cbrt(a: f64) -> f64 {
	a.cbrt()
}

pub extern "C" fn nrt(a: f64, b: f64) -> f64 {
	a.powf(1.0 / b)
}

pub extern "C" fn pow(a: f64, b: f64) -> f64 {
	a.powf(b)
}

pub extern "C" fn graph(f: f64) {

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
		.draw_series(LineSeries::new(
			(-10..=10).map(|x| (x as f32, 0.0)),
			&BLACK,
		))
		.unwrap();

	chart
		.draw_series(LineSeries::new(
			(-10..=10).map(|x| (0.0, x as f32)),
			&BLACK,
		))
		.unwrap();

	chart
		.configure_series_labels()
		.background_style(WHITE.mix(0.8))
		.border_style(BLACK)
		.draw()
		.unwrap();

	root.present().unwrap();
}
