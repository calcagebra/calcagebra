use crate::token::{Token, TokenInfo};

pub struct Lexer<'a> {
	contents: &'a str,
}

impl<'a> Lexer<'a> {
	pub fn new(contents: &'a str) -> Self {
		Self { contents }
	}

	pub fn tokens(&self) -> Vec<Vec<TokenInfo>> {
		let mut offset = 1;
		let mut tokeninfos = vec![];

		for line in self.contents.lines() {
			if !line.starts_with("//") && !line.is_empty() {
				let tokens = self.tokenize_line(line, offset);

				if let Some(token) = tokens.last() {
					offset += token.range.end;
				}

				tokeninfos.push(tokens);
			} else {
				offset += 1;
			}
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
				if !token.is_empty() {
					let size = token.len();

					tokens.push(TokenInfo::new(token, c..c + size));
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

				let size = token.len();

				tokens.push(TokenInfo::new(token, c..c + size));

				c += size;
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

				let size = token.len();

				tokens.push(TokenInfo::new(token, c..c + size));

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

		let mut r = vec![];

		let mut offset = 0;
		let mut token_iter = tokens.into_iter().peekable();

		loop {
			let tokeninfo = token_iter.peek();

			if tokeninfo.is_none() {
				break;
			}

			let tokeninfo = token_iter.next().unwrap();

			let start = tokeninfo.range.start;
			let end = tokeninfo.range.end;

			if token_iter.peek().is_none() {
				r.push(tokeninfo.set_range(start..end + offset));

				break;
			}

			match tokeninfo.token {
				Token::Float(..) => {
					r.push(tokeninfo.set_range(start..end + offset));

					if let Token::Ident(..) = token_iter.peek().unwrap().token {
						offset += 1;

						r.push(TokenInfo {
							token: Token::Mul,
							range: start..end + offset,
						});
					}
				}
				_ => {
					r.push(tokeninfo.set_range(start..end + offset));
				}
			}
		}

		r
	}
}
