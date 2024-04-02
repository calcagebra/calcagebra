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
                if token.chars().all(|a| a.is_ascii_digit()) {
                    Token::Number(token.parse::<f32>().unwrap())
                } else {
                    Token::Identifier(token)
                }
            }
        }
    }
}
