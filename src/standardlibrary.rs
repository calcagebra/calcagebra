use std::io::{stdin, stdout, Write};

// IO
pub unsafe extern "C" fn print(value: f64) -> f64 {
	println!("{}", value);
	0.0
}

pub unsafe extern "C" fn read() -> f64 {
	print!("Enter number: ");
	stdout().flush().unwrap();
	let mut buf = String::new();

	stdin().read_line(&mut buf).unwrap();

	buf.trim_end().parse::<f64>().unwrap()
}

// MATH

pub unsafe extern "C" fn round(a: f64) -> f64 {
    a.round()
}

pub unsafe extern "C" fn ceil(a: f64) -> f64 {
    a.ceil()
}

pub unsafe extern "C" fn floor(a: f64) -> f64 {
    a.floor()
}

pub unsafe extern "C" fn ln(a: f64) -> f64 {
    a.ln()
}

pub unsafe extern "C" fn log10(a: f64) -> f64 {
    a.log10()
}

pub unsafe extern "C" fn log(a: f64, b: f64) -> f64 {
    a.log(b)
}

pub unsafe extern "C" fn sin(a: f64) -> f64 {
    a.sin()
}

pub unsafe extern "C" fn cos(a: f64) -> f64 {
    a.cos()
}

pub unsafe extern "C" fn tan(a: f64) -> f64 {
    a.tan()
}

pub unsafe extern "C" fn sqrt(a: f64) -> f64 {
    a.sqrt()
}

pub unsafe extern "C" fn cbrt(a: f64) -> f64 {
    a.cbrt()
}

pub unsafe extern "C" fn nrt(a: f64, b: f64) -> f64 {
    a.powf(1.0 / b)
}

pub unsafe extern "C" fn pow(a: f64, b: f64) -> f64 {
	a.powf(b)
}
