use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Number(f32),
    Identifier(String),

    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Pow,

    Comma,
    LParen,
    RParen,
}

impl Token {
    pub fn new(token: String) -> Self {
        match token.as_str() {
            "=" => Token::Eq,
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "^" => Token::Pow,

            "," => Token::Comma,
            "(" => Token::LParen,
            ")" => Token::RParen,

            _ => {
                if token.chars().all(|a| a.is_ascii_digit() || a=='.') {
                    Token::Number(token.parse::<f32>().unwrap())
                } else {
                    Token::Identifier(token)
                }
            }
        }
    }
}


impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Number(x) => x.to_string(),
            Token::Identifier(ident) => ident.to_string(),
            Token::Eq => "=".to_string(),
            Token::Add => "+".to_string(),
            Token::Sub => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Pow => "^".to_string(),
            Token::Comma => ",".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
        })
    }
}