use std::collections::HashMap;
use std::ops::RangeInclusive;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::{
	term,
	term::termcolor::{ColorChoice, StandardStream},
};

use crate::ast::AstType;

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
		range: RangeInclusive<usize>,
		(expected, got): (AstType, AstType),
	) {
		let diagnostic = Diagnostic::error()
			.with_message("incompatible types")
			.with_code("E101")
			.with_labels(vec![Label::primary(
				*self.file_ids.get(file).unwrap(),
				*range.start()..*range.end()+1,
			)
			.with_message(format!("expected `{expected}`, found `{got}`"))])
			.with_notes(vec![format!(
				"help: use `{expected}(...)` method to convert to correct type"
			)]);

		let writer = StandardStream::stderr(ColorChoice::Always);
		let config = codespan_reporting::term::Config::default();

		term::emit(&mut writer.lock(), &config, &self.files, &diagnostic).unwrap();
	}
}
