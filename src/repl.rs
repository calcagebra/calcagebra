use core::mem;
use rustyline::{
	error::ReadlineError, highlight::Highlighter, validate::MatchingBracketValidator, Completer,
	Config, Editor, Helper, Hinter, Validator,
};
use std::{
	borrow::Cow::{self, Borrowed, Owned},
	fmt::Write,
	process::exit,
};
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

use crate::{errors::ErrorReporter, jit::Jit, lexer::Lexer, parser::Parser, version};

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

	let mut jit = Jit::default();

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

				let mut reporter = ErrorReporter::new();
				reporter.add_file("REPL", &line);

				jit.renew();

				unsafe {
					mem::transmute::<*const u8, fn()>(
						jit
							.execute(
								Parser::new("REPL", Lexer::new(&line).tokens(), reporter).ast(),
								false,
							)
							.unwrap(),
					)()
				};
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
				println!("Error: {:?}", err);
				break;
			}
		}
	}
	exit(1);
}
