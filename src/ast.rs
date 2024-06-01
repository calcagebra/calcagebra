use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum Ast {
    Assignment(String, Expression),
    FunctionCall(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<String>, Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Abs(Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Branched(Box<Expression>, Box<Expression>, Box<Expression>),
    Differentiate(Box<Expression>),
    Identifier(String),
    Number(f32),
    SizedSet(Vec<Expression>),
    UnsizedSet(Vec<Expression>, Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
    Undefined,
}

impl Expression {
    pub fn differentiate(&self, args: &[String]) -> Expression {
        match self {
            Expression::Binary(e1, op, e2) => match op {
                Token::Add | Token::Sub => Expression::Binary(
                    Box::new(e1.differentiate(args)),
                    op.to_owned(),
                    Box::new(e2.differentiate(args)),
                ),
                Token::Mul => Expression::Binary(
                    Box::new(Expression::Binary(
                        Box::new(e1.differentiate(args)),
                        Token::Mul,
                        e2.to_owned(),
                    )),
                    Token::Add,
                    Box::new(Expression::Binary(
                        e1.to_owned(),
                        Token::Mul,
                        Box::new(e2.differentiate(args)),
                    )),
                ),
                Token::Div => Expression::Binary(
                    Box::new(Expression::Binary(
                        Box::new(Expression::Binary(
                            Box::new(e1.differentiate(args)),
                            Token::Mul,
                            e2.to_owned(),
                        )),
                        Token::Sub,
                        Box::new(Expression::Binary(
                            e1.to_owned(),
                            Token::Mul,
                            Box::new(e2.differentiate(args)),
                        )),
                    )),
                    Token::Div,
                    Box::new(Expression::Binary(
                        e2.to_owned(),
                        Token::Pow,
                        Box::new(Expression::Number(2.0)),
                    )),
                ),
                Token::Pow => Expression::Binary(
                    e2.to_owned(),
                    Token::Mul,
                    Box::new(Expression::Binary(
                        e1.to_owned(),
                        Token::Pow,
                        Box::new(match *e2.to_owned() {
                            Expression::Identifier(_) => Expression::Binary(
                                e2.to_owned(),
                                Token::Sub,
                                Box::new(Expression::Number(1.0)),
                            ),
                            Expression::Number(n) => *Box::new(Expression::Number(n - 1.0)),
                            _ => unimplemented!(),
                        }),
                    )),
                ),
                _ => unimplemented!(),
            },
            Expression::Branched(_, _, _) => todo!(),
            Expression::Differentiate(expr) => expr.differentiate(args).differentiate(args),
            Expression::Identifier(ident) => {
                if args.contains(ident) {
                    Expression::Number(1.0)
                } else {
                    Expression::Number(0.0)
                }
            }
            Expression::Number(_) => Expression::Number(0.0),

            Expression::Abs(_)
            | Expression::SizedSet(_)
            | Expression::UnsizedSet(_, _)
            | Expression::FunctionCall(_, _) => unimplemented!(),
            Expression::Undefined => todo!(),
        }.flatten()
    }

    pub fn flatten(&self) -> Expression {
        match self {
            Expression::Binary(e1, op, e2) => match (*e1.to_owned(), op, *e2.to_owned()) {
                (Expression::Number(a), Token::Add, Expression::Number(b)) => {
                    Expression::Number(a + b)
                }
                (Expression::Number(a), Token::Sub, Expression::Number(b)) => {
                    Expression::Number(a - b)
                }
                (Expression::Number(a), Token::Mul, Expression::Number(b)) => {
                    Expression::Number(a * b)
                }
                (Expression::Number(a), Token::Div, Expression::Number(b)) => {
                    Expression::Number(a / b)
                }
                (Expression::Number(a), Token::Pow, Expression::Number(b)) => {
                    Expression::Number(a.powf(b))
                }
                (Expression::Number(a), Token::Mul, Expression::Binary(b, Token::Mul, c)) => {
                    match (*b.to_owned(), *c.to_owned()) {
                        (Expression::Number(x), Expression::Number(y)) => {
                            Expression::Number(a * x * y)
                        }
                        (Expression::Number(x), d) => Expression::Binary(
                            Box::new(Expression::Number(x * a)),
                            Token::Mul,
                            Box::new(d),
                        ),
                        (d, Expression::Number(x)) => Expression::Binary(
                            Box::new(Expression::Number(x * a)),
                            Token::Mul,
                            Box::new(d),
                        ),
                        _ => Expression::Binary(
                            Box::new(Expression::Number(a)),
                            Token::Mul,
                            Box::new(Expression::Binary(b, Token::Mul, c)),
                        ),
                    }
                }
                _ => Expression::Binary(
                    Box::new(e1.flatten()),
                    op.to_owned(),
                    Box::new(e2.flatten()),
                ),
            },
            _ => self.to_owned(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Abs(expr) => format!("|{expr}|"),
                Expression::Binary(e1, op, e2) => {
                    if *op == Token::Mul {
                        if Expression::Number(0.0) == *e1.to_owned()
                            || Expression::Number(0.0) == *e2.to_owned()
                        {
                            String::new()
                        } else if Expression::Number(1.0) == *e1.to_owned() {
                            format!("{e2}")
                        } else if Expression::Number(1.0) == *e2.to_owned() {
                            format!("{e1}")
                        } else {
                            format!("{e1}{e2}")
                        }
                    } else if *op == Token::Pow {
                        if Expression::Number(0.0) == *e2.to_owned() {
                            String::from("1")
                        } else if Expression::Number(1.0) == *e2.to_owned() {
                            format!("{e1}")
                        } else {
                            format!("{e1}{op}{e2}")
                        }
                    } else {
                        let s1 = format!("{e1}");
                        let s2 = format!("{e2}");

                        if s1.is_empty() {
                            s2
                        } else if s2.is_empty() {
                            s1
                        } else {
                            format!("{e1}{op}{e2}")
                        }
                    }
                }
                Expression::Branched(e1, e2, e3) => format!("if {e1} then {e2} else {e3} end"),
                Expression::Differentiate(f) => format!("d/dx {f}"),
                Expression::Identifier(ident) => ident.to_string(),
                Expression::Number(n) => n.to_string(),
                Expression::SizedSet(s) => format!(
                    "{{ {} }}",
                    s.iter()
                        .map(|f| format!("{f}"))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Expression::UnsizedSet(s1, s2) => format!(
                    "{{ {} : {} }}",
                    s1.iter()
                        .map(|f| format!("{f}"))
                        .collect::<Vec<_>>()
                        .join(","),
                    s2.iter()
                        .map(|f| format!("{f}"))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Expression::FunctionCall(ident, args) => format!(
                    "{ident}({})",
                    args.iter()
                        .map(|f| format!("{f}"))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                Expression::Undefined => "UNDEFINED".to_string(),
            }
        )
    }
}
