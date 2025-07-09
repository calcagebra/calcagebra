use crate::token::{Token, TokenInfo};

pub struct Lexer<'a> {
	contents: &'a str,
}

impl<'a> Lexer<'a> {
	#[inline(always)]
	pub fn new(contents: &'a str) -> Self {
		Self { contents }
	}

	#[inline(always)]
	pub fn tokens(&self) -> Vec<Vec<TokenInfo>> {
		let mut location = 1;
		let mut tokeninfos = vec![];

		for line in self.contents.lines() {
			if line.starts_with("//") || line.is_empty() {
				location += 1;

				continue;
			}

			let tokens = self.tokenize_line(line, location);

			if let Some(token) = tokens.last() {
				location += token.range.end;
			}

			tokeninfos.push(tokens);
		}

		tokeninfos
	}

	fn tokenize_line(&self, line: &str, mut c: usize) -> Vec<TokenInfo> {
		let mut line = line.chars().peekable();
		let mut tokens = vec![];

		loop {
			let mut token = String::new();
			let char = line.next();

			if char.is_none() {
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

				let size = token.len();

				tokens.push(TokenInfo::new(token, c..c + size));

				c += size;
			} else if char.is_ascii_digit() {
				token.push(char);
				let to_insert_mul;

				loop {
					let char = line.peek();

					if char.is_none() || (!char.unwrap().is_ascii_digit() && *char.unwrap() != '.') {
						to_insert_mul = char.is_some() && char.unwrap().is_ascii_alphabetic();
						break;
					}

					let char = line.next();

					token.push(char.unwrap());
				}

				let size = token.len();

				tokens.push(TokenInfo::new(token, c..c + size));

				if to_insert_mul {
					tokens.push(TokenInfo {
						token: Token::Mul,
						range: c..c + 1,
					});
					c += 1;
				}

				c += size;
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
				let size = token.len();

				tokens.push(TokenInfo::new(token, c..c + size));

				c += size;
			}
		}

		tokens
	}
}
