use std::{collections::HashSet, fmt::Display};

use super::Data;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct Set {
    pub values: Vec<Data>,
}

impl Set {
    pub fn new(values: Vec<Data>) -> Self {
        Self {
            values: values
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
        }
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.values
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
