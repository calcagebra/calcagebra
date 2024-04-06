use std::fmt::Display;

use super::Data;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct SizedSet {
    pub values: Vec<Data>,
}

impl SizedSet {
    pub fn new(args: Vec<Data>) -> Self {
        let mut values = vec![];
        for arg in args {
            if !values.contains(&arg) {
                values.push(arg)
            }
        }
        Self { values }
    }
}

impl Display for SizedSet {
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
