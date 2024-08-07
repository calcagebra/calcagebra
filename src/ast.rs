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
    Identifier(String),
    Number(f64),
    FunctionCall(String, Vec<Expression>),
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
                Expression::Identifier(ident) => ident.to_string(),
                Expression::Number(n) => n.to_string(),
                Expression::FunctionCall(ident, args) => format!(
                    "{ident}({})",
                    args.iter()
                        .map(|f| format!("{f}"))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            }
        )
    }
}
