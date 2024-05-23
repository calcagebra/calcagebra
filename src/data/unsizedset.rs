use std::cmp::Ordering;

use crate::{
    ast::Expression,
    interpreter::{Functions, Interpreter, Std, Variables},
    token::Token,
};

use super::{sizedset::SizedSet, Data};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BaseSet {
    Natural,
    Whole,
    Integer,
    Real,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Conditions {
    Lt(Expression),
    LtEq(Expression),
    IsEq(Expression),
    NEq(Expression),
    GtEq(Expression),
    Gt(Expression),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataConditions {
    Lt(f32),
    LtEq(f32),
    IsEq(f32),
    NEq(f32),
    GtEq(f32),
    Gt(f32),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UnsizedSet {
    conditions: Vec<Conditions>,
    base_set: BaseSet,
}

impl UnsizedSet {
    pub fn new(idents: Vec<Expression>, exprs: Vec<Expression>) -> Self {
        let mut conditions = vec![];
        let mut base_set = BaseSet::Real;

        for expr in exprs {
            if let Expression::Binary(lhs, op, rhs) = expr {
                let operand = if idents.contains(&lhs) { rhs } else { lhs };

                match op {
                    Token::IsEq => conditions.push(Conditions::IsEq(*operand)),
                    Token::NEq => conditions.push(Conditions::NEq(*operand)),
                    Token::Gt => conditions.push(Conditions::Gt(*operand)),
                    Token::Lt => conditions.push(Conditions::Lt(*operand)),
                    Token::GtEq => conditions.push(Conditions::GtEq(*operand)),
                    Token::LtEq => conditions.push(Conditions::LtEq(*operand)),
                    Token::Belongs => {
                        base_set = if let Expression::Identifier(i) = *operand {
                            match i.as_str() {
                                "N" => {
                                    conditions.push(Conditions::GtEq(Expression::Number(1.0)));
                                    BaseSet::Natural
                                }
                                "W" => {
                                    conditions.push(Conditions::GtEq(Expression::Number(0.0)));
                                    BaseSet::Whole
                                }
                                "I" | "Z" => BaseSet::Integer,
                                "R" => BaseSet::Real,
                                _ => unreachable!(),
                            }
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        Self {
            conditions,
            base_set,
        }
    }

    pub fn sorted_conditions(
        &self,
        variables: &Variables,
        functions: &Functions,
        std: &Std,
    ) -> Vec<DataConditions> {
        let mut conditions = self
            .conditions
            .iter()
            .map(|condition| match condition {
                Conditions::Lt(expr) => DataConditions::Lt(
                    Interpreter::eval_expression(expr, variables, functions, std).to_number(),
                ),
                Conditions::LtEq(expr) => DataConditions::LtEq(
                    Interpreter::eval_expression(expr, variables, functions, std).to_number(),
                ),
                Conditions::IsEq(expr) => DataConditions::IsEq(
                    Interpreter::eval_expression(expr, variables, functions, std).to_number(),
                ),
                Conditions::NEq(expr) => DataConditions::NEq(
                    Interpreter::eval_expression(expr, variables, functions, std).to_number(),
                ),
                Conditions::GtEq(expr) => DataConditions::GtEq(
                    Interpreter::eval_expression(expr, variables, functions, std).to_number(),
                ),
                Conditions::Gt(expr) => DataConditions::Gt(
                    Interpreter::eval_expression(expr, variables, functions, std).to_number(),
                ),
            })
            .collect::<Vec<DataConditions>>();
        conditions.sort_by(|a, b| {
            let x = match a {
                DataConditions::Lt(f) => f,
                DataConditions::LtEq(f) => f,
                DataConditions::IsEq(f) => f,
                DataConditions::NEq(f) => f,
                DataConditions::GtEq(f) => f,
                DataConditions::Gt(f) => f,
            };

            let y = match b {
                DataConditions::Lt(f) => f,
                DataConditions::LtEq(f) => f,
                DataConditions::IsEq(f) => f,
                DataConditions::NEq(f) => f,
                DataConditions::GtEq(f) => f,
                DataConditions::Gt(f) => f,
            };

            if let DataConditions::NEq(_) = a {
                Ordering::Greater
            } else if let DataConditions::NEq(_) = b {
                Ordering::Less
            } else if x > y {
                Ordering::Greater
            } else if x == y {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        });

        conditions
    }

    pub fn len(&self, variables: &Variables, functions: &Functions, std: &Std) -> f32 {
        let mut size = 0.0;
        let mut last_condition = None;

        for condition in &self.sorted_conditions(variables, functions, std) {
            if last_condition.is_none() {
                last_condition = Some(condition);
                continue;
            }

            match (condition, last_condition.unwrap()) {
                (DataConditions::Lt(d1), DataConditions::Gt(d2)) => {
                    if d1 > d2 {
                        size += d1 - d2 - 1.0
                    }
                }
                (DataConditions::Lt(d1), DataConditions::GtEq(d2)) => {
                    if d1 > d2 {
                        size += d1 - d2
                    }
                }
                (DataConditions::LtEq(d1), DataConditions::Gt(d2)) => {
                    if d1 > d2 {
                        size += d1 - d2
                    }
                }
                (DataConditions::LtEq(d1), DataConditions::GtEq(d2)) => {
                    if d1 > d2 {
                        size += d1 - d2 + 1.0
                    }
                }
                (DataConditions::GtEq(d1), DataConditions::Lt(d2)) => {
                    if d2 > d1 {
                        size += d1 - d2
                    }
                }
                (DataConditions::GtEq(d1), DataConditions::LtEq(d2)) => {
                    if d2 > d1 {
                        size += d1 - d2 + 1.0
                    }
                }
                (DataConditions::Gt(d1), DataConditions::Lt(d2)) => {
                    if d2 > d1 {
                        size += d1 - d2 - 1.0
                    }
                }
                (DataConditions::Gt(d1), DataConditions::LtEq(d2)) => {
                    if d2 > d1 {
                        size += d1 - d2
                    }
                }
                _ => {}
            }

            last_condition = Some(condition);
        }
        size
    }

    pub fn to_sizedset(&self, variables: &Variables, functions: &Functions, std: &Std) -> SizedSet {
        let mut values = vec![];
        let mut last_condition = None;

        for condition in &self.sorted_conditions(variables, functions, std) {
            if last_condition.is_none() {
                last_condition = Some(condition);
                continue;
            }

            match (condition, last_condition.unwrap()) {
                (DataConditions::Lt(d1), DataConditions::Gt(d2)) => {
                    if d1 > d2 {
                        for x in *d2 as i32 + 1..*d1 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::Lt(d1), DataConditions::GtEq(d2)) => {
                    if d1 > d2 {
                        for x in *d2 as i32 + 1..=*d1 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::LtEq(d1), DataConditions::Gt(d2)) => {
                    if d1 > d2 {
                        for x in *d2 as i32..*d1 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::LtEq(d1), DataConditions::GtEq(d2)) => {
                    if d1 > d2 {
                        for x in *d2 as i32..=*d1 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::GtEq(d1), DataConditions::Lt(d2)) => {
                    if d2 > d1 {
                        for x in *d1 as i32..*d2 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::GtEq(d1), DataConditions::LtEq(d2)) => {
                    if d2 > d1 {
                        for x in *d1 as i32..=*d2 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::Gt(d1), DataConditions::Lt(d2)) => {
                    if d2 > d1 {
                        for x in *d1 as i32 + 1..*d2 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::Gt(d1), DataConditions::LtEq(d2)) => {
                    if d2 > d1 {
                        for x in *d1 as i32 + 1..=*d2 as i32 {
                            values.push(Data::Number(x as f32));
                        }
                    }
                }
                (DataConditions::IsEq(d), _) | (_, DataConditions::IsEq(d)) => {
                    if !values.contains(&Data::Number(*d)) {
                        values.push(Data::Number(*d))
                    }
                }
                (DataConditions::NEq(d), _) | (_, DataConditions::NEq(d)) => {
                    if values.contains(&Data::Number(*d)) {
                        let index = values
                            .binary_search_by(|x| x.partial_cmp(&Data::Number(*d)).unwrap())
                            .unwrap();
                        values.remove(index);
                    }
                }
                _ => {}
            }

            last_condition = Some(condition);
        }

        SizedSet { values }
    }
}
