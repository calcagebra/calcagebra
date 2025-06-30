use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::{
	term,
	term::termcolor::{ColorChoice, StandardStream},
};

use crate::token::Token;
use crate::types::NumberType;

pub enum ParserError {
	SyntaxError(SyntaxError),
	TypeError(TypeError),
	LogicError(String),
}

impl From<&str> for ParserError {
	fn from(value: &str) -> Self {
		ParserError::LogicError(value.to_string())
	}
}

impl ParserError {
	pub fn error_message(&self) -> String {
		match self {
			ParserError::SyntaxError(syntax_error) => syntax_error.error_message(),
			ParserError::TypeError(type_error) => type_error.error_message(),
			ParserError::LogicError(error_message) => error_message.to_string(),
		}
	}

	pub fn help_message(&self) -> String {
		match self {
			ParserError::SyntaxError(syntax_error) => syntax_error.help_message(),
			ParserError::TypeError(type_error) => type_error.help_message(),
			ParserError::LogicError(help_message) => help_message.to_string(),
		}
	}

	pub fn range(&self) -> Range<usize> {
		match self {
			ParserError::SyntaxError(syntax_error) => syntax_error.range.clone(),
			ParserError::TypeError(type_error) => type_error.range.clone(),
			ParserError::LogicError(..) => 0..0,
		}
	}
}

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

	pub fn to_parser_error(self) -> ParserError {
		ParserError::SyntaxError(self)
	}
}

pub struct TypeError {
	expected: NumberType,
	got: NumberType,
	pub range: Range<usize>,
}

impl TypeError {
	pub fn new(expected: NumberType, got: NumberType, range: Range<usize>) -> Self {
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

	pub fn to_parser_error(self) -> ParserError {
		ParserError::TypeError(self)
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
			.with_code("ERR")
			.with_labels(vec![
				Label::primary((), range.start - 1..range.end).with_message(error_message),
			])
			.with_notes(vec![help_message]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.file, &diagnostic).unwrap();

		std::process::exit(1);
	}
}
