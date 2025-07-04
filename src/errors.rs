use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::{
	term,
	term::termcolor::{ColorChoice, StandardStream},
};

use crate::token::Token;
use crate::types::DataType;

#[derive(Debug)]
pub enum Error {
	SyntaxError(SyntaxError),
	TypeError(TypeError),
	LogicError(String),
	EOLError(EOLError),
}

impl From<&str> for Error {
	fn from(value: &str) -> Self {
		Error::LogicError(value.to_string())
	}
}

impl Error {
	pub fn error_message(&self) -> String {
		match self {
			Error::SyntaxError(syntax_error) => syntax_error.error_message(),
			Error::TypeError(type_error) => type_error.error_message(),
			Error::LogicError(error_message) => error_message.to_string(),
			Error::EOLError(eol_error) => eol_error.error_message(),
		}
	}

	pub fn help_message(&self) -> String {
		match self {
			Error::SyntaxError(syntax_error) => syntax_error.help_message(),
			Error::TypeError(type_error) => type_error.help_message(),
			Error::LogicError(help_message) => help_message.to_string(),
			Error::EOLError(eol_error) => eol_error.help_message(),
		}
	}

	pub fn range(&self) -> Range<usize> {
		match self {
			Error::SyntaxError(syntax_error) => syntax_error.range.clone(),
			Error::TypeError(type_error) => type_error.range.clone(),
			Error::LogicError(..) => 0..0,
			Error::EOLError(eol_error) => eol_error.range.clone(),
		}
	}
}

#[derive(Debug)]
pub struct SyntaxError {
	expected: Token,
	got: Token,
	pub range: Range<usize>,
}

impl SyntaxError {
	pub fn new(expected: Token, got: Token, range: Range<usize>) -> Self {
		Self {
			expected,
			got,
			range,
		}
	}

	pub fn error_message(&self) -> String {
		format!(
			"\x1b[1mencountered `{}` where {} was expected \x1b[0m",
			self.got, self.expected
		)
	}

	pub fn help_message(&self) -> String {
		format!("\x1b[1mhelp:\x1b[0m add {} here", self.expected)
	}

	pub fn to_error(self) -> Error {
		Error::SyntaxError(self)
	}
}

#[derive(Debug)]
pub struct TypeError {
	expected: DataType,
	got: DataType,
	pub range: Range<usize>,
}

impl TypeError {
	pub fn new(expected: DataType, got: DataType, range: Range<usize>) -> Self {
		Self {
			expected,
			got,
			range,
		}
	}

	pub fn error_message(&self) -> String {
		format!(
			"\x1b[1mexpected `{}`, found `{}`\x1b[0m",
			self.expected, self.got
		)
	}

	pub fn help_message(&self) -> String {
		format!(
			"\x1b[1mhelp:\x1b[0m use `{}(...)` method to convert to correct type",
			self.expected
		)
	}

	pub fn to_error(self) -> Error {
		Error::TypeError(self)
	}
}

#[derive(Debug)]
pub struct EOLError {
	pub range: Range<usize>,
}

impl EOLError {
	pub fn new(range: Range<usize>) -> Self {
		Self { range }
	}

	pub fn error_message(&self) -> String {
		"\x1b[1munexpected end of tokens \x1b[0m".to_string()
	}

	pub fn help_message(&self) -> String {
		"\x1b[1mhelp:\x1b[0m more tokens were expected here".to_string()
	}

	pub fn to_error(self) -> Error {
		Error::EOLError(self)
	}
}

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

	pub fn error(&self, error_message: String, help_message: String, range: Range<usize>) -> ! {
		let diagnostic = Diagnostic::error()
			.with_message(&error_message)
			.with_labels(vec![
				Label::primary((), range.start.saturating_sub(1)..range.end).with_message(error_message),
			])
			.with_notes(vec![help_message]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.file, &diagnostic).unwrap();

		std::process::exit(1);
	}

	pub fn error_without_exit(
		&self,
		error_message: String,
		help_message: String,
		range: Range<usize>,
	) {
		let diagnostic = Diagnostic::error()
			.with_message(&error_message)
			.with_labels(vec![
				Label::primary((), range.start.saturating_sub(1)..range.end).with_message(error_message),
			])
			.with_notes(vec![help_message]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.file, &diagnostic).unwrap();
	}
}
