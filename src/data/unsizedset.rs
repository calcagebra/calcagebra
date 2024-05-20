use crate::{
    ast::Expression,
    interpreter::{Functions, Interpreter, Std, Variables},
    token::Token,
};

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
                    Token::HashTag => {
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

        // TODO: Sort and merge conditions that can be merged

        Self {
            conditions,
            base_set,
        }
    }

    pub fn len(&self, variables: &Variables, functions: &Functions, std: &Std) -> f32 {
        let mut size = 0.0;
        let mut prev_condition = None;
        for condition in &self.conditions {
            if prev_condition.is_none() {
                prev_condition = Some(condition);
                continue;
            }

            match (condition, prev_condition.unwrap()) {
                (Conditions::Lt(e1), Conditions::Gt(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d1 > d2 {
                        size += d1 - d2 - 1.0
                    }
                }
                (Conditions::Lt(e1), Conditions::GtEq(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d1 > d2 {
                        size += d1 - d2
                    }
                }
                (Conditions::LtEq(e1), Conditions::Gt(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d1 > d2 {
                        size += d1 - d2
                    }
                }
                (Conditions::LtEq(e1), Conditions::GtEq(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d1 > d2 {
                        size += d1 - d2 + 1.0
                    }
                }
                (Conditions::GtEq(e1), Conditions::Lt(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d2 > d1 {
                        size += d1 - d2
                    }
                }
                (Conditions::GtEq(e1), Conditions::LtEq(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d2 > d1 {
                        size += d1 - d2 + 1.0
                    }
                }
                (Conditions::Gt(e1), Conditions::Lt(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d2 > d1 {
                        size += d1 - d2 - 1.0
                    }
                }
                (Conditions::Gt(e1), Conditions::LtEq(e2)) => {
                    let d1 = Interpreter::eval_expression(e1, variables, functions, std)
                        .to_number();
                    let d2 = Interpreter::eval_expression(e2, variables, functions, std)
                        .to_number();

                    if d2 > d1 {
                        size += d1 - d2
                    }
                }
                _ => unimplemented!(),
            }

            prev_condition = Some(condition);
        }
        size
    }
}
