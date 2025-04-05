use std::{fmt::Display, ops::RangeInclusive};

#[derive(Debug, Clone)]
pub struct TokenInfo {
	pub token: Token,
	pub range: RangeInclusive<usize>,
}

impl TokenInfo {
	pub fn new(token: Token, range: RangeInclusive<usize>) -> Self {
		Self { token, range }
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	Float(f32),
	Integer(i32),
	Identifier(String),

	Let,
	Fn,
	If,
	Then,
	Else,
	End,
	Import,

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
	LSquare,
	RSquare,
	LCurly,
	RCurly,
	Abs,
}

impl Token {
	pub fn new(token: String) -> Self {
		match token.as_str().trim() {
			"let" => Token::Let,
			"fn" => Token::Fn,
			"if" => Token::If,
			"then" => Token::Then,
			"else" => Token::Else,
			"end" => Token::End,
			"import" => Token::Import,

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
			"[" => Token::LSquare,
			"]" => Token::RSquare,
			"{" => Token::LCurly,
			"}" => Token::RCurly,
			_ => {
				if token.chars().all(|a| a.is_ascii_digit() || a == '.') {
					let try_integer = token.parse::<i32>();
					let try_float = token.parse::<f32>();

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

	pub fn is_operator(&self) -> bool {
		match self {
			Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Pow | Token::Rem => true,
			_ => false,
		}
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Token::Integer(n) => n.to_string(),
				Token::Float(n) => n.to_string(),
				Token::Identifier(ident) => ident.to_string(),
				Token::Let => "let".to_string(),
				Token::Fn => "fn".to_string(),
				Token::If => "if".to_string(),
				Token::Then => "then".to_string(),
				Token::Else => "else".to_string(),
				Token::End => "end".to_string(),
				Token::Import => "import".to_string(),
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
				Token::Belongs => "E".to_string(),
				Token::Colon => ":".to_string(),
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
