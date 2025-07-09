use crate::{
	errors::Error, interpreter::InterpreterContext, lexer::Lexer, parser::Parser, types::Data,
};
use std::io::{Write, stdin, stdout};

#[inline(always)]
pub fn print(a: Vec<Data>) -> Data {
	for b in a {
		println!("{b}");
	}
	Data::new_zero()
}

#[inline(always)]
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
		.parser(&mut tokens[0].iter().peekable(), 0)
		.unwrap()
		.0
		.evaluate(ctx, 0..0)
}
