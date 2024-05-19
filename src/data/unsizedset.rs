use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BaseSet {
    Natural,
    Whole,
    Integers,
    Real,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UnsizedSet {
    upper_limit: f32,
    lower_limit: f32,
    base_set: BaseSet,
}

impl UnsizedSet {
    pub fn new(idents: Vec<Expression>, conditions: Vec<Expression>) -> Self {
        Self {
            upper_limit: 0.0,
            lower_limit: 0.0,
            base_set: BaseSet::Real
        }
    }
}