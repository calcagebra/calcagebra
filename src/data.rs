use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};

#[derive(Debug, Clone)]
pub enum Data {
    Number(f32),
    Function(String),
}

impl Data {
    pub fn default() -> Self {
        Data::Number(0.0)
    }
    pub fn to_number(&self) -> f32 {
        match self {
            Data::Number(n) => *n,
            Data::Function(_) => unimplemented!(),
        }
    }

    pub fn to_function(&self) -> String {
        match self {
            Data::Number(_) => unimplemented!(),
            Data::Function(ident) => ident.to_string(),
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Data::Number(n) => n.to_string(),
            Data::Function(ident) => ident.to_string(),
        })
    }
}

impl Add for Data {
    type Output = Data;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n + m),
            (Data::Number(_), Data::Function(_)) => unimplemented!(),
            (Data::Function(_), Data::Number(_)) => unimplemented!(),
            (Data::Function(_), Data::Function(_)) => todo!(),
        }
    }
}

impl Sub for Data {
    type Output = Data;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n - m),
            (Data::Number(_), Data::Function(_)) => unimplemented!(),
            (Data::Function(_), Data::Number(_)) => unimplemented!(),
            (Data::Function(_), Data::Function(_)) => todo!(),
        }
    }
}

impl Mul for Data {
    type Output = Data;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n * m),
            (Data::Number(_), Data::Function(_)) => unimplemented!(),
            (Data::Function(_), Data::Number(_)) => unimplemented!(),
            (Data::Function(_), Data::Function(_)) => todo!(),
        }
    }
}

impl Div for Data {
    type Output = Data;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n / m),
            (Data::Number(_), Data::Function(_)) => unimplemented!(),
            (Data::Function(_), Data::Number(_)) => unimplemented!(),
            (Data::Function(_), Data::Function(_)) => todo!(),
        }
    }
}

