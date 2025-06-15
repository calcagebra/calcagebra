mod ast;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod repl;
mod standardlibrary;
mod token;
mod types;

use std::{fs::read_to_string, time::Instant};

use clap::{Parser as ClapParser, Subcommand, command};
use errors::ErrorReporter;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use repl::repl;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Output debug information
	#[clap(short, long, value_parser, global = true)]
	debug: bool,

	/// Print the time elapsed while executing code
	#[clap(short, long, value_parser, global = true)]
	time: bool,

	#[command(subcommand)]
	command: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
	/// Build calcagebra binary and then execute it
	#[command(arg_required_else_help = true)]
	Run {
		/// Name of the file to run
		name: String,
	},

	Repl,
}

fn main() {
	let args = Args::parse();

	let input = match args.command {
		Subcommands::Run { name } => name,
		Subcommands::Repl => String::new(),
	};

	if input.is_empty() {
		repl();
	}

	run(&input, args.debug, args.time);
}

pub fn version() -> String {
	env!("CARGO_PKG_VERSION").to_string()
}

pub fn run(input: &str, debug: bool, time: bool) {
	let main = Instant::now();

	let contents = read_to_string(input).unwrap();

	let reporter = ErrorReporter::new(input, &contents);

	let tokens = Lexer::new(&contents).tokens();

	if debug {
		let duration = main.elapsed();
		println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
	}

	let ast = Parser::new(tokens, reporter).ast();

	if debug {
		let duration = main.elapsed();
		println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
	}

	Interpreter::new().interpret(ast);

	if debug || time {
		let duration = main.elapsed();
		println!("\nTIME: {duration:?}");
	}
}

#[cfg(test)]
mod tests {
    use crate::run;

	#[test]
	fn matrix() {
		run("tests/matrix.cal", false, false);
	}
}
