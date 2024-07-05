use std::fmt::Display;

use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub expr: Expression,
}

impl Function {
    pub fn new(name: String, args: Vec<String>, expr: Expression) -> Self {
        Self { name, args, expr }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) = {}",
            self.name,
            self.args
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(","),
            self.expr
        )
    }
}
