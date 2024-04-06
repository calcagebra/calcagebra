pub mod set;

use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

use self::set::Set;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Data {
    Number(f32),
    Function(String),
    Set(Set),
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

    pub fn to_function(&self) -> String {
        match self {
            Data::Function(ident) => ident.to_string(),
            _ => unimplemented!(),
        }
    }

    pub fn to_set(&self) -> &Set {
        match self {
            Data::Set(s) => s,
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
                Data::Function(ident) => ident.to_string(),
                Data::Set(set) => set.to_string(),
            }
        )
    }
}

impl Add for Data {
    type Output = Data;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n + m),
            (Data::Function(_), Data::Function(_)) => todo!(),
            (_, _) => unimplemented!(),
        }
    }
}

impl Sub for Data {
    type Output = Data;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n - m),
            (_, _) => unimplemented!(),
        }
    }
}

impl Mul for Data {
    type Output = Data;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n * m),
            (Data::Number(n), Data::Set(s)) | (Data::Set(s), Data::Number(n)) => {
                Data::Set(Set::new(
                    s.values
                        .into_iter()
                        .map(|f| (f * Data::Number(n)))
                        .collect::<Vec<Data>>(),
                ))
            }
            (_, _) => unimplemented!(),
        }
    }
}

impl Div for Data {
    type Output = Data;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Data::Number(n), Data::Number(m)) => Data::Number(n / m),
            (Data::Number(n), Data::Set(s)) | (Data::Set(s), Data::Number(n)) => {
                Data::Set(Set::new(
                    s.values
                        .into_iter()
                        .map(|f| (f / Data::Number(n)))
                        .collect::<Vec<Data>>(),
                ))
            }
            (_, _) => unimplemented!(),
        }
    }
}
