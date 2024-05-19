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
                tokens.push(Token::new(token.clone()));
                token.clear();
            } else if char.is_ascii_digit() {
                token.push(char);
                loop {
                    let char = line.peek();

                    if char.is_none()
                        || (!char.unwrap().is_ascii_digit() && *char.unwrap() != '.')
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
                let punctuation = ['.', '(', ')', '{', '}', '[', ']', '|'];
                loop {
                    let char = line.peek();

                    if char.is_none()
                        || char.unwrap().is_ascii_alphanumeric()
                        || punctuation.contains(char.unwrap())
                        || punctuation.map(|f| token.contains(f)).contains(&true)
                    {
                        break;
                    }

                    let char = line.next();

                    token.push(char.unwrap());
                }
                tokens.push(Token::new(token.clone()));
                token.clear();
            }
        }

        let mut r = vec![];

        for i in 0..tokens.len() {
            r.push(tokens[i].clone());

            if tokens.get(i + 1).is_none() {
                break;
            }

            if let Token::Number(_) = tokens[i] {
                if let Token::Identifier(_) = tokens[i + 1] {
                    r.push(Token::Mul);
                }
            }
        }

        r
    }
}
