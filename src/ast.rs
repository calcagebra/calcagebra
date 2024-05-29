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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Abs(expr) => format!("|{expr}|"),
                Expression::Binary(e1, op, e2) => format!(
                    "{e1}{}{e2}",
                    if *op == Token::Mul {
                        String::new()
                    } else {
                        op.to_string()
                    }
                ),
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
                Expression::Undefined => "UNDEFINED".to_string()
            }
        )
    }
}
