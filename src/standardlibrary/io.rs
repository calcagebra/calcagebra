use std::io::{Write, stdin, stdout};

use crate::{interpreter::InterpreterContext, lexer::Lexer, parser::Parser, types::Data};

pub fn print(a: Vec<Data>) -> Data {
	for b in a {
		println!("{b}");
	}
	Data::Number(0.0, 0.0)
}

pub fn read<'a>(ctx: &'a mut InterpreterContext<'a>) -> (&'a mut InterpreterContext<'a>, Data) {
	print!("Enter value: ");

	stdout().flush().unwrap();
	let mut buf = String::new();

	stdin().read_line(&mut buf).unwrap();

	let tokens = Lexer::new(buf.trim_end()).tokens();

	Parser::new(&tokens)
		.parser(tokens[0].iter().peekable(), 0)
		.unwrap()
		.0
		.evaluate(ctx)
}
