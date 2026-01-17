use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TreeColor {
    #[default]
    Red,
    Black,
}

impl Display for TreeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "B"),
            Self::Red => write!(f, "R"),
        }
    }
}
