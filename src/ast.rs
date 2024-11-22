use crate::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum AstNode {
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