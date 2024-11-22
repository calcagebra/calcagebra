mod ast;
mod jit;
mod lexer;
mod parser;
mod token;

use core::mem;
use std::{fs::read_to_string, time::Instant};

use clap::{command, Parser as ClapParser, Subcommand};
use jit::Jit;
use lexer::Lexer;

use crate::parser::Parser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Output debug information
	#[clap(short, long, value_parser, global = true)]
	debug: bool,

	/// Emit llvm ir
	#[clap(short, long, value_parser, global = true)]
	emit_ir: bool,

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
}

fn main() {
	let args = Args::parse();
	let main = Instant::now();

	let input = match args.command {
		Subcommands::Run { name } => name,
	};

	let contents = read_to_string(input.clone()).unwrap();

	let tokens = Lexer::new(&contents).tokens();

	if args.debug {
		let duration = main.elapsed();
		println!("LEXER: {tokens:?}\n\nTIME: {duration:?}\n");
	}

	let ast = Parser::new(tokens).ast();

	if args.debug {
		let duration = main.elapsed();
		println!("AST: {ast:?}\n\nTIME: {duration:?}\n");
	}

	let mut jit = Jit::default();

	unsafe { mem::transmute::<*const u8, fn()>(jit.execute(ast).unwrap())() };

	if args.debug || args.time {
		let duration = main.elapsed();
		println!("\nTIME: {duration:?}");
	}
}

pub fn version() -> String {
	env!("CARGO_PKG_VERSION").to_string()
}
