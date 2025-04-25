use std::collections::HashMap;
use std::ops::RangeInclusive;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::{
	term,
	term::termcolor::{ColorChoice, StandardStream},
};

use crate::token::Token;
use crate::types::NumberType;

#[derive(Debug, Clone)]
pub struct ErrorReporter {
	files: SimpleFiles<String, String>,
	file_ids: HashMap<String, usize>,
}

impl ErrorReporter {
	pub fn new() -> Self {
		Self {
			files: SimpleFiles::new(),
			file_ids: HashMap::new(),
		}
	}

	pub fn add_file(&mut self, name: &str, content: &str) {
		let fileid = self
			.files
			.add(name.to_string(), unindent::unindent(content));
		self.file_ids.insert(name.to_string(), fileid);
	}

	pub fn type_error(
		&self,
		file: &str,
		range: &RangeInclusive<usize>,
		(expected, got): (NumberType, NumberType),
	) {
		let diagnostic = Diagnostic::error()
			.with_message("incompatible types")
			.with_code("E101")
			.with_labels(vec![
				Label::primary(
					*self.file_ids.get(file).unwrap(),
					*range.start() - 1..*range.end() - 1,
				)
				.with_message(format!(
					"\x1b[1mexpected `{expected}`, found `{got}`\x1b[0m"
				)),
			])
			.with_notes(vec![format!(
				"\x1b[1mhelp:\x1b[0m use `{expected}(...)` method to convert to correct type"
			)]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).unwrap();
	}

	pub fn syntax_error(
		&self,
		file: &str,
		range: &RangeInclusive<usize>,
		(expected, got): (&Token, &Token),
	) -> ! {
		let diagnostic = Diagnostic::error()
			.with_message("syntax error")
			.with_code("E201")
			.with_labels(vec![
				Label::primary(
					*self.file_ids.get(file).unwrap(),
					*range.start() - 1..*range.end() - 1,
				)
				.with_message(format!(
					"\x1b[1mencountered `{got}` where {expected} was expected \x1b[0m"
				)),
			])
			.with_notes(vec![format!("\x1b[1mhelp:\x1b[0m add {expected} here")]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).unwrap();

		std::process::exit(1)
	}
}
