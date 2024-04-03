use crate::token::Token;

pub struct Lexer<'a> {
    contents: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self { contents }
    }

    pub fn tokens(&self) -> Vec<Vec<Token>> {
        self.contents
            .lines()
            .map(|line| {
                if !line.starts_with("//") {
                    self.tokenize_line(line)
                } else {
                    vec![]
                }
            })
            .filter(|x| !x.is_empty())
            .collect()
    }

    fn tokenize_line(&self, line: &str) -> Vec<Token> {
        let mut line = line.chars().peekable();
        let mut tokens = vec![];
        let mut token = String::new();

        loop {
            let char = line.next();

            if char.is_none() {
                if !token.is_empty() {
                    tokens.push(Token::new(token));
                }
                break;
            }

            let char = char.unwrap();

            if char.is_whitespace() {
                continue;
            }

            if char.is_ascii_alphanumeric() || char == '$' {
                token.push(char);
                loop {
                    let char = line.peek();

                    if char.is_none()
                        || (!char.unwrap().is_ascii_alphanumeric()
                            && (*char.unwrap() != '.' || *char.unwrap() != '$'))
                    {
                        break;
                    }

                    let char = line.next();

                    token.push(char.unwrap());
                }
                tokens.push(Token::new(token.clone()));
                token.clear();
            } else {
                token.push(char);
                tokens.push(Token::new(token.clone()));
                token.clear();
            }
        }

        tokens
    }
}
