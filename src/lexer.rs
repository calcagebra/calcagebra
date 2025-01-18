use crate::token::{Token, TokenInfo};
use std::ops::RangeInclusive;

pub struct Lexer<'a> {
	contents: &'a str,
}

impl<'a> Lexer<'a> {
	pub fn new(contents: &'a str) -> Self {
		Self { contents }
	}

	pub fn tokens(&self) -> Vec<Vec<TokenInfo>> {
		self
			.contents
			.lines()
			.enumerate()
			.map(|(i, line)| {
				if !line.starts_with("//") {
					self.tokenize_line(line, i)
				} else {
					vec![]
				}
			})
			.filter(|x| !x.is_empty())
			.collect()
	}

	fn tokenize_line(&self, line: &str, line_number: usize) -> Vec<TokenInfo> {
		let mut line = line.chars().peekable();
		let mut tokens = vec![];
		let mut token = String::new();

		let mut c = 1;

		loop {
			let char = line.next();

			if char.is_none() {
				if !token.is_empty() {
					tokens.push(TokenInfo::new(
						Token::new(token.clone()),
						line_number,
						range_from_size(c, token.len()),
					));
				}
				break;
			}

			let char = char.unwrap();

			c += 1;

			if char.is_whitespace() {
				continue;
			}

			if char.is_ascii_alphabetic() {
				token.push(char);
				loop {
					let char = line.peek();

					if char.is_none() || !char.unwrap().is_ascii_alphabetic() {
						break;
					}

					let char = line.next();

					c += 1;

					token.push(char.unwrap());
				}
				tokens.push(TokenInfo::new(
					Token::new(token.clone()),
					line_number,
					range_from_size(c, token.len()),
				));
				token.clear();
			} else if char.is_ascii_digit() {
				token.push(char);
				loop {
					let char = line.peek();

					if char.is_none() || (!char.unwrap().is_ascii_digit() && *char.unwrap() != '.') {
						break;
					}

					let char = line.next();

					c += 1;

					token.push(char.unwrap());
				}
				tokens.push(TokenInfo::new(
					Token::new(token.clone()),
					line_number,
					range_from_size(c, token.len()),
				));
				token.clear();
			} else {
				token.push(char);
				let punctuation = ['.', '(', ')', '{', '}', '[', ']', '|'];
				loop {
					let char = line.peek();

					if char.is_none()
						|| char.unwrap().is_ascii_alphanumeric()
						|| punctuation.contains(char.unwrap())
						|| punctuation.map(|f| token.contains(f)).contains(&true)
					{
						break;
					}

					let char = line.next();

					c += 1;

					token.push(char.unwrap());
				}
				tokens.push(TokenInfo::new(
					Token::new(token.clone()),
					line_number,
					range_from_size(c, token.len()),
				));
				token.clear();
			}
		}

		let mut r = vec![];

		let mut offset = 0;

		for i in 0..tokens.len() {
            let tokeninfo = tokens.remove(i);
            
			r.push(TokenInfo::new(
				tokeninfo.token.clone(),
				tokeninfo.line,
				*tokeninfo.range.start()..=tokeninfo.range.end() + offset,
			));

			if tokens.get(i + 1).is_none() {
				break;
			}

			if let Token::Integer(_) = tokeninfo.token {
				if let Token::Identifier(_) = tokens[i + 1].token {
					offset += 1;
					r.push(TokenInfo::new(
						Token::Mul,
						line_number,
						*tokeninfo.range.start()..=tokeninfo.range.end() + offset,
					));
				}
			}
		}

		r
	}
}

fn range_from_size(start: usize, size: usize) -> RangeInclusive<usize> {
	start..=start + size
}
