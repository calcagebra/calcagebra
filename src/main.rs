mod ast;
mod lexer;
mod parser;
mod repl;
mod standardlibrary;
mod token;
mod errors;
mod interpreter;
mod types;

use std::{fs::read_to_string, time::Instant};

use clap::{command, Parser as ClapParser, Subcommand};
use errors::ErrorReporter;
use lexer::Lexer;
use interpreter::Interpreter;
use repl::repl;
use parser::Parser;

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
	let main = Instant::now();

	let input = match args.command {
		Subcommands::Run { name } => name,
		Subcommands::Repl => String::new(),
	};

	if input.is_empty() {
		repl();
	}

	let contents = read_to_string(input.clone()).unwrap();

	let mut reporter = ErrorReporter::new();

	reporter.add_file(&input, &contents);

	let tokens = Lexer::new(&contents).tokens();

	if args.debug {
		let duration = main.elapsed();
		println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
	}

	let ast = Parser::new(&input, tokens, reporter).ast();

	if args.debug {
		let duration = main.elapsed();
		println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
	}

	Interpreter::new().interpret(ast);

	if args.debug || args.time {
		let duration = main.elapsed();
		println!("\nTIME: {duration:?}");
	}
}

pub fn version() -> String {
	env!("CARGO_PKG_VERSION").to_string()
}
