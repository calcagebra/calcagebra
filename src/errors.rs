use std::ops::RangeInclusive;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::{
	term,
	term::termcolor::{ColorChoice, StandardStream},
};

use crate::token::Token;
use crate::types::NumberType;

#[derive(Debug, Clone)]
pub struct ErrorReporter<'a> {
	file: SimpleFile<&'a str, &'a str>,
}

impl<'a> ErrorReporter<'a> {
	pub fn new(name: &'a str, source: &'a str) -> Self {
		Self {
			file: SimpleFile::new(name, source),
		}
	}

	pub fn type_error(
		&self,
		range: &RangeInclusive<usize>,
		(expected, got): (NumberType, NumberType),
	) {
		let diagnostic = Diagnostic::error()
			.with_message("incompatible types")
			.with_code("E101")
			.with_labels(vec![
				Label::primary((), *range.start() - 1..*range.end() - 1).with_message(format!(
					"\x1b[1mexpected `{expected}`, found `{got}`\x1b[0m"
				)),
			])
			.with_notes(vec![format!(
				"\x1b[1mhelp:\x1b[0m use `{expected}(...)` method to convert to correct type"
			)]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.file, &diagnostic).unwrap();
	}

	pub fn syntax_error(
		&self,
		range: &RangeInclusive<usize>,
		(expected, got): (&Token, &Token),
	) -> ! {
		let diagnostic = Diagnostic::error()
			.with_message("syntax error")
			.with_code("E201")
			.with_labels(vec![
				Label::primary((), *range.start() - 1..*range.end() - 1).with_message(format!(
					"\x1b[1mencountered `{got}` where {expected} was expected \x1b[0m"
				)),
			])
			.with_notes(vec![format!("\x1b[1mhelp:\x1b[0m add {expected} here")]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.file, &diagnostic).unwrap();

		std::process::exit(1)
	}
}
