pub mod errors;
pub mod expr;
pub mod interpreter;
pub mod lexer;
pub mod parser;
mod standardlibrary;
mod token;
mod types;

use std::{fs::read_to_string, time::Instant};

use errors::ErrorReporter;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub use standardlibrary::io::print;

pub fn version() -> String {
	env!("CARGO_PKG_VERSION").to_string()
}

pub fn run(input: &str, debug: bool, time: bool) {
	let contents = read_to_string(input).unwrap();

	let main = Instant::now();

	let reporter = ErrorReporter::new(input, &contents);

	let tokens = Lexer::new(&contents).tokens();

	if debug {
		let duration = main.elapsed();
		println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
	}

	let ast = match Parser::new(&tokens).ast() {
		Ok(ast) => ast,
		Err(err) => reporter.error(err.error_message(), err.help_message(), err.range()),
	};

	if debug {
		let duration = main.elapsed();
		println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
	}

	match Interpreter::new().interpret(ast) {
		Ok(..) => (),
		Err(err) => reporter.error(err.error_message(), err.help_message(), err.range()),
	}

	if debug || time {
		let duration = main.elapsed();
		println!("\nTIME: {duration:?}");
	}
}

#[cfg(test)]
mod tests {
	use crate::run;

	#[test]
	fn assignment() {
		run("tests/assignment.cal", false, false);
	}

	#[test]
	fn function_declaration() {
		run("tests/function_declaration.cal", false, false);
	}

	#[test]
	fn matrix() {
		run("tests/matrix.cal", false, false);
	}
}
