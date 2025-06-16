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
		let mut offset = 1;

		self
			.contents
			.lines()
			.map(|line| {
				if !line.starts_with("//") && !line.is_empty() {
					let tokens = self.tokenize_line(line, offset);

					if let Some(token) = tokens.last() {
						offset += *token.range.end();
					}

					tokens
				} else {
					offset += 1;
					vec![]
				}
			})
			.filter(|x| !x.is_empty())
			.collect()
	}

	fn tokenize_line(&self, line: &str, mut c: usize) -> Vec<TokenInfo> {
		let mut line = line.chars().peekable();
		let mut tokens = vec![];
		let mut token = String::new();

		loop {
			let char = line.next();

			if char.is_none() {
				if !token.is_empty() {
					tokens.push(TokenInfo::new(
						Token::new(token.clone()),
						range_from_size(c, token.len()),
					));
				}
				break;
			}

			let char = char.unwrap();

			if char.is_whitespace() {
				c += 1;
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

					token.push(char.unwrap());
				}
				tokens.push(TokenInfo::new(
					Token::new(token.clone()),
					range_from_size(c, token.len()),
				));
				c += token.len();
				token.clear();
			} else if char.is_ascii_digit() {
				token.push(char);
				loop {
					let char = line.peek();

					if char.is_none() || (!char.unwrap().is_ascii_digit() && *char.unwrap() != '.') {
						break;
					}

					let char = line.next();

					token.push(char.unwrap());
				}
				tokens.push(TokenInfo::new(
					Token::new(token.clone()),
					range_from_size(c, token.len()),
				));
				c += token.len();
				token.clear();
			} else {
				token.push(char);
				let punctuation = ['.', '(', ')', '{', '}', '[', ']', '|', ','];
				loop {
					let char = line.peek();

					if char.is_none()
						|| char.unwrap().is_ascii_alphanumeric()
						|| char.unwrap().is_whitespace()
						|| punctuation.contains(char.unwrap())
						|| punctuation.map(|f| token.contains(f)).contains(&true)
					{
						break;
					}

					let char = line.next();

					token.push(char.unwrap());
				}
				tokens.push(TokenInfo::new(
					Token::new(token.clone()),
					range_from_size(c, token.len()),
				));
				c += token.len();
				token.clear();
			}
		}

		let mut r = vec![];

		let mut offset = 0;

		for i in 0..tokens.len() {
			let tokeninfo = tokens.get(i).unwrap();

			r.push(TokenInfo::new(
				tokeninfo.token.clone(),
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
