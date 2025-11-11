use std::{
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

use miette::Diagnostic;
use thiserror::Error;

use crate::QueryRule;

/// Represents errors that can occur during the parsing phase.
#[derive(Error, Debug, Diagnostic)]
pub enum ParsingError {
    #[error(transparent)]
    Pest(#[from] Box<pest::error::Error<QueryRule>>),

    #[error(transparent)]
    Int(#[from] ParseIntError),

    #[error(transparent)]
    Float(#[from] ParseFloatError),

    #[error(transparent)]
    Bool(#[from] ParseBoolError),

    #[error("Unexpected rule: {rule:?}")]
    UnexpectedRule { rule: QueryRule },
}
