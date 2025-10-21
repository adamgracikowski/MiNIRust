use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Const {
    Numeric(i64),
    Named(String),
}

impl Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Const::Numeric(number) => write!(f, "{number}"),
            Const::Named(name) => write!(f, "{name}"),
        }
    }
}
