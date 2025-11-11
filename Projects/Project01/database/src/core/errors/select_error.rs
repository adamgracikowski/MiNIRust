use miette::Diagnostic;
use thiserror::Error;

/// Represents errors that can occur during the execution of a `SELECT` statement.
#[derive(Error, Debug, Diagnostic)]
pub enum SelectError {
    #[error("Invalid LIMIT value: {limit}. Value must be non-negative.")]
    InvalidLimit { limit: i64 },
}
