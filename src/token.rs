#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Number(i32),
    Identifier(String),

    Eq,
    Neq,
    GT,
    LT,
    Geq,
    Leq,

    Add,
    Sub,
    Mul,
    Div,
    Pow,

    LParen,
    RParen,
}

impl Token {
    pub fn new(token: String) -> Self {
        match token.as_str() {
            "=" => Token::Eq,
            "!=" => Token::Neq,
            ">" => Token::GT,
            "<" => Token::LT,
            ">=" => Token::Geq,
            "<=" => Token::Leq,

            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "^" => Token::Pow,

            "(" => Token::LParen,
            ")" => Token::RParen,

            _ => {
                if token.chars().all(|a| a.is_ascii_digit()) {
                    Token::Number(token.parse::<i32>().unwrap())
                } else {
                    Token::Identifier(token)
                }
            }
        }
    }
}
