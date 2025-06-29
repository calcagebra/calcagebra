use std::io::{Write, stdin, stdout};

use crate::types::Number;

pub fn print(a: Vec<Number>) -> Number {
	for b in a {
		println!("{b}");
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
