mod lexer;
mod token;
mod ast;
mod parser;
mod interpreter;

use std::fs::read_to_string;

use clap::Parser as ClapParser;
use lexer::Lexer;

use crate::{interpreter::Interpreter, parser::Parser};

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    /// Whether to dry run
    #[clap(long, value_parser)]
    dry_run: bool,

    /// Whether to print the time elapsed
    #[clap(short, long, value_parser)]
    time: bool,

    /// The code which is to be executed
    #[clap(short, long, value_parser)]
    code: Option<String>,

    /// The path of file which is to be executed
    #[clap()]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let contents = read_to_string(args.input.unwrap()).unwrap();

    let tokens = Lexer::new(&contents).tokens();

    let ast = Parser::new(tokens).ast();

    Interpreter::new(ast).run();
}
