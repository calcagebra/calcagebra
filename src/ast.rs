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
    Number(i32),
    FunctionCall(String, Vec<Expression>)
}
