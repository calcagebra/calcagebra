use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]

pub enum Ast {
    Assignment(String, Expression),
    FunctionCall(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<String>, Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Identifier(String),
    Number(f32),
    SizedSet(Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Binary(lhs, op, rhs) =>
                    lhs.to_string() + &op.to_string() + &rhs.to_string(),
                Expression::Identifier(ident) => ident.to_string(),
                Expression::Number(n) => n.to_string(),
                Expression::SizedSet(exprs) => exprs
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                Expression::FunctionCall(ident, exprs) => {
                    ident.to_string()
                        + &exprs
                            .iter()
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                }
            }
        )
    }
}
