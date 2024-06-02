use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub expr: Expression,
}

impl Function {
    pub fn new(name: String, args: Vec<String>, expr: Expression) -> Self {
        Self {
            name,
            args,
            expr
        }
    }
}