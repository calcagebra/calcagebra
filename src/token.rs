#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Number(f32),
    Identifier(String),

    If,
    Then,
    Else,
    End,

    Eq,
    
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
    Modulo,

    Comma,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
}

impl Token {
    pub fn new(token: String) -> Self {
        match token.as_str().trim() {
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "end" => Token::End,

            "=" => Token::Eq,

            "==" => Token::IsEq,
            ">" => Token::Gt,
            "<" => Token::Lt,
            ">=" => Token::GtEq,
            "<=" => Token::LtEq,

            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "^" => Token::Pow,
            "%" => Token::Modulo,

            "," => Token::Comma,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "{" => Token::LCurly,
            "}" => Token::RCurly,
            "[" => Token::LSquare,
            "]" => Token::RSquare,
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