use std::io::{Write, stdin, stdout};

use rust_decimal::Decimal;

use crate::{
	errors::Error, interpreter::InterpreterContext, lexer::Lexer, parser::Parser, types::Data,
};

pub fn print(a: Vec<Data>) -> Data {
	for b in a {
		println!("{b}");
	}
	Data::new_zero()
}

pub fn read<'a, 'b>(ctx: &'a mut InterpreterContext<'b>) -> Result<Data, Error>
where
	'b: 'a,
{
	print!("Enter value: ");

	stdout().flush().unwrap();
	let mut buf = String::new();

	stdin().read_line(&mut buf).unwrap();

	let tokens = Lexer::new(buf.trim_end()).tokens();

	Parser::new(&tokens)
		.parser(tokens[0].iter().peekable(), 0)
		.unwrap()
		.0
		.evaluate(ctx, 0..0)
}
