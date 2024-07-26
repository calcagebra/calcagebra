pub mod sizedset;
pub mod unsizedset;
pub mod function;

use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::interpreter::{Functions, Std, Variables};

use self::{function::Function, sizedset::SizedSet, unsizedset::UnsizedSet};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Data {
    Number(f32),
    Bool(bool),
    Function(Function),
    SizedSet(SizedSet),
    UnsizedSet(UnsizedSet),
}

impl Data {
    pub fn default() -> Self {
        Data::Number(0.0)
    }
    pub fn to_number(&self) -> f32 {
        match self {
            Data::Number(n) => *n,
            _ => unimplemented!(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Data::Bool(n) => *n,
            _ => unimplemented!(),
        }
    }

    pub fn to_function(&self) -> String {
        match self {
            Data::Function(f) => f.name.to_string(),
            _ => unimplemented!(),
        }
    }

    pub fn to_set(&self, variables: &Variables, functions: &Functions, std: &Std) -> SizedSet {
        match self {
            Data::SizedSet(s) => s.clone(),
            Data::UnsizedSet(s) => s.to_sizedset(variables, functions, std),
            _ => unimplemented!(),
        }
    }
}

impl Eq for Data {}

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Data::Number(n) => n.to_string(),
                Data::Bool(b) => b.to_string(),
                Data::Function(f) => format!(
                    "{} {} = {}",
                    f.name,
                    f.args.iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    f.expr
                ),
                Data::SizedSet(set) => set.to_string(),
                _ => unimplemented!(),
            }
        )
    }
}

impl Add for Data {
    type Output = Data;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n + m),
            (..) => unimplemented!(),
        }
    }
}

impl Sub for Data {
    type Output = Data;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n - m),
            _ => unimplemented!(),
        }
    }
}

impl Mul for Data {
    type Output = Data;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n * m),
            (Data::Number(n), Data::SizedSet(s)) | (Data::SizedSet(s), Data::Number(n)) => {
                Data::SizedSet(SizedSet::new(
                    s.values
                        .into_iter()
                        .map(|f| (f * Data::Number(n)))
                        .collect::<Vec<Data>>(),
                ))
            }
            _ => unimplemented!(),
        }
    }
}

impl Div for Data {
    type Output = Data;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n / m),
            (Data::Number(n), Data::SizedSet(s)) | (Data::SizedSet(s), Data::Number(n)) => {
                Data::SizedSet(SizedSet::new(
                    s.values
                        .into_iter()
                        .map(|f| (f / Data::Number(n)))
                        .collect::<Vec<Data>>(),
                ))
            }
            _ => unimplemented!(),
        }
    }
}

impl Rem for Data {
    type Output = Data;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n % m),
            (Data::Number(n), Data::SizedSet(s)) | (Data::SizedSet(s), Data::Number(n)) => {
                Data::SizedSet(SizedSet::new(
                    s.values
                        .into_iter()
                        .map(|f| (f % Data::Number(n)))
                        .collect::<Vec<Data>>(),
                ))
            }
            _ => unimplemented!(),
        }
    }
}
