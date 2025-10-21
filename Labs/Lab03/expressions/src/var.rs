use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Var {
    X,
    Y,
    Z,
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Var::X => write!(f, "X"),
            Var::Y => write!(f, "Y"),
            Var::Z => write!(f, "Z"),
        }
    }
}
