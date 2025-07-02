use std::io::{Write, stdin, stdout};

use crate::{
	interpreter::{Interpreter, InterpreterContext},
	lexer::Lexer,
	parser::Parser,
	types::Number,
};

pub fn print(a: Vec<Number>) -> Number {
	for b in a {
		println!("{b}");
	}
	Number::Real(0.0)
}

pub fn read(ctx: &mut InterpreterContext) -> Number {
	print!("Enter value: ");

	stdout().flush().unwrap();
	let mut buf = String::new();

	stdin().read_line(&mut buf).unwrap();

	let tokens = Lexer::new(buf.trim_end()).tokens();

	Interpreter::interpret_expression(
		ctx,
		&Parser::new(&tokens)
			.pratt_parser(tokens[0].iter().peekable(), 0)
			.unwrap()
			.0,
	)
}
