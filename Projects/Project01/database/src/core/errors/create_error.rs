use miette::Diagnostic;
use thiserror::Error;

/// Represents errors that can occur during the execution of a `CREATE` statement.
#[derive(Error, Debug, Diagnostic)]
pub enum CreateError {
    #[error("Table '{name}' already exists")]
    TableAlreadyExists { name: String },
}
