use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct TokenInfo {
	pub token: Token,
	pub line: usize,
	pub range: RangeInclusive<usize>,
}

impl TokenInfo {
	pub fn new(token: Token, line: usize, range: RangeInclusive<usize>) -> Self {
		Self { token, line, range }
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	Float(f64),
	Integer(i64),
	Identifier(String),

	If,
	Then,
	Else,
	End,

	Eq,

	NEq,
	IsEq,
	Gt,
	Lt,
	GtEq,
	LtEq,

	Add,
	Sub,
	Mul,
	Div,
	Pow,
	Rem,

	Comma,
	Belongs,
	Colon,
	LParen,
	RParen,
	LCurly,
	RCurly,
	Abs,
}

impl Token {
	pub fn new(token: String) -> Self {
		match token.as_str().trim() {
			"if" => Token::If,
			"then" => Token::Then,
			"else" => Token::Else,
			"end" => Token::End,

			"=" => Token::Eq,
			"!=" => Token::NEq,
			"==" => Token::IsEq,
			">" => Token::Gt,
			"<" => Token::Lt,
			">=" => Token::GtEq,
			"<=" => Token::LtEq,

			"|" => Token::Abs,
			"+" => Token::Add,
			"-" => Token::Sub,
			"*" => Token::Mul,
			"/" => Token::Div,
			"^" => Token::Pow,
			"%" => Token::Rem,

			"," => Token::Comma,
			"E" => Token::Belongs,
			":" => Token::Colon,
			"(" => Token::LParen,
			")" => Token::RParen,
			"{" => Token::LCurly,
			"}" => Token::RCurly,
			_ => {
				if token.chars().all(|a| a.is_ascii_digit() || a == '.') {
					let try_integer = token.parse::<i64>();
					let try_float = token.parse::<f64>();

					if let Ok(n) = try_integer {
						Token::Integer(n)
					} else if let Ok(f) = try_float {
						Token::Float(f)
					} else {
						panic!("could not lex number: `{}`", token);
					}
				} else {
					Token::Identifier(token)
				}
			}
		}
	}
}
