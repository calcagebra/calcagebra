#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Number(f64),
    Identifier(String),

    If,
    Then,
    Else,
    End,
    Import,
    From,

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
            "import" => Token::Import,
            "from" => Token::From,

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
                    Token::Number(token.parse::<f64>().unwrap())
                } else {
                    Token::Identifier(token)
                }
            }
        }
    }
}
