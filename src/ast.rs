use crate::token::Token;

#[derive(Debug, Clone)]

pub enum Ast {
    Assignment(String, Expression),
    FunctionCall(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<String>, Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Abs(Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Branched(Box<Expression>, Box<Expression>, Box<Expression>),
    Identifier(String),
    Number(f32),
    SizedSet(Vec<Expression>),
    UnsizedSet(Vec<Expression>,Vec<Expression>),
    FunctionCall(String, Vec<Expression>),
}
