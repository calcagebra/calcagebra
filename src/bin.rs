use clap::{Parser as ClapParser, Subcommand, command};

use rustyline::{
	Completer, Config, Editor, Helper, Hinter, Validator, error::ReadlineError,
	highlight::Highlighter, validate::MatchingBracketValidator,
};
use std::{
	borrow::Cow::{self, Borrowed, Owned},
	fmt::Write,
	process::exit,
};
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

use calcagebra_lib::{
	errors::ErrorReporter, expr::Expression, interpreter::Interpreter, lexer::Lexer, parser::Parser,
	print, run, version,
};

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

#[derive(Helper, Completer, Hinter, Validator)]
struct HighlightHelper {
	#[rustyline(Validator)]
	validator: MatchingBracketValidator,
	colored_prompt: String,
}

impl Highlighter for HighlightHelper {
	fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
		&'s self,
		prompt: &'p str,
		default: bool,
	) -> Cow<'b, str> {
		if default {
			Borrowed(&self.colored_prompt)
		} else {
			Borrowed(prompt)
		}
	}

	fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
		Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
	}

	fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
		let ps = SyntaxSet::load_defaults_newlines();
		let ts = ThemeSet::load_defaults();
		let theme = ts.themes["base16-mocha.dark"].clone();

		let syntax = ps.find_syntax_by_extension("rs").unwrap();
		let mut highlighter = HighlightLines::new(syntax, &theme);

		let highlighted = highlighter
			.highlight_line(line, &ps)
			.unwrap()
			.into_iter()
			.fold(String::new(), |mut acc, (style, text)| {
				let _ = write!(
					acc,
					"\x1b[38;2;{};{};{}m{}\x1b[0m",
					style.foreground.r, style.foreground.g, style.foreground.b, text
				);

				acc
			});

		Cow::Owned(highlighted)
	}

	fn highlight_char(&self, _: &str, _: usize, _: bool) -> bool {
		true
	}
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

pub fn repl() {
	println!(
		"Welcome to calcagebra v{}\nTo exit, press CTRL+C or CTRL+D",
		version()
	);

	let config = Config::builder().build();

	let h = HighlightHelper {
		colored_prompt: "".to_owned(),
		validator: MatchingBracketValidator::new(),
	};
	let mut rl = Editor::with_config(config).unwrap();
	rl.set_helper(Some(h));

	let mut interpreter = Interpreter::new();
	let mut ctx = &mut (&mut interpreter.globals, &mut interpreter.functions);

	loop {
		"\x1b[1m\x1b[32m[In]:\x1b[0m "
			.clone_into(&mut rl.helper_mut().expect("No helper").colored_prompt);

		let readline = rl.readline("\x1b[1m\x1b[32m[In]:\x1b[0m ");

		match readline {
			Ok(line) => {
				if line.trim() == "exit" {
					break;
				}

				println!("\x1b[1m\x1b[31m[Out]:\x1b[0m ");

				let reporter = ErrorReporter::new("REPL", &line);

				let tokens = Lexer::new(&line).tokens();

				let parser = Parser::new(&tokens);

				match parser.ast() {
					Ok(ast) => {
						if !ast.is_empty() {
							let data;

							(ctx, data) = match ast[0].clone().0.evaluate(ctx, ast[0].1.clone()) {
								Ok(tuple) => tuple,
								Err(err) => {
									reporter.error_without_exit(err.error_message(), err.help_message(), err.range()),
								}
							};

							if let Expression::FunctionCall(name, _) = &ast[0].0
								&& name == "print"
							{
								continue;
							}

							print(vec![data]);
						}
					}
					Err(err) => {
						reporter.error_without_exit(err.error_message(), err.help_message(), err.range())
					}
				}
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			}
			Err(err) => {
				println!("Error: {err:?}");
				break;
			}
		}
	}
	exit(1);
}
