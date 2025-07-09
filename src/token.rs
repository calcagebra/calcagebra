use rust_decimal::Decimal;
use std::{fmt::Display, ops::Range, str::FromStr};

#[derive(Debug, Clone)]
pub struct TokenInfo {
	pub token: Token,
	pub range: Range<usize>,
}

impl TokenInfo {
	#[inline(always)]
	pub fn new(token: String, range: Range<usize>) -> Self {
		Self {
			token: Token::new(token),
			range,
		}
	}

	#[inline(always)]
	pub fn set_range(mut self, range: Range<usize>) -> Self {
		self.range = range;

		self
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	Float(Decimal),
	Ident(String),

	Let,
	Fn,
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
	Colon,
	Semi,
	LParen,
	RParen,
	LSquare,
	RSquare,
	LCurly,
	RCurly,
	Abs,
}

impl Token {
	#[inline(always)]
	pub fn new(token: String) -> Self {
		match token.as_ref() {
			"let" => Token::Let,
			"fn" => Token::Fn,
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
			":" => Token::Colon,
			";" => Token::Semi,
			"(" => Token::LParen,
			")" => Token::RParen,
			"[" => Token::LSquare,
			"]" => Token::RSquare,
			"{" => Token::LCurly,
			"}" => Token::RCurly,
			_ => {
				let try_decimal = Decimal::from_str(&token);

				if let Ok(f) = try_decimal {
					Token::Float(f)
				} else {
					Token::Ident(token)
				}
			}
		}
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Token::Float(n) => n.to_string(),
				Token::Ident(ident) => ident.to_string(),
				Token::Let => "let".to_string(),
				Token::Fn => "fn".to_string(),
				Token::If => "if".to_string(),
				Token::Then => "then".to_string(),
				Token::Else => "else".to_string(),
				Token::End => "end".to_string(),
				Token::Eq => "=".to_string(),
				Token::NEq => "!=".to_string(),
				Token::IsEq => "==".to_string(),
				Token::Gt => ">".to_string(),
				Token::Lt => "<".to_string(),
				Token::GtEq => ">=".to_string(),
				Token::LtEq => "<=".to_string(),
				Token::Abs => "|".to_string(),
				Token::Add => "+".to_string(),
				Token::Sub => "-".to_string(),
				Token::Mul => "*".to_string(),
				Token::Div => "/".to_string(),
				Token::Pow => "^".to_string(),
				Token::Rem => "%".to_string(),
				Token::Comma => ",".to_string(),
				Token::Colon => ":".to_string(),
				Token::Semi => ";".to_string(),
				Token::LParen => "(".to_string(),
				Token::RParen => ")".to_string(),
				Token::LSquare => "[".to_string(),
				Token::RSquare => "]".to_string(),
				Token::LCurly => "{".to_string(),
				Token::RCurly => "}".to_string(),
			}
		)
	}
}
