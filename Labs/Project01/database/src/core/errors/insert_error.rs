use miette::Diagnostic;
use thiserror::Error;

/// Represents errors that can occur during the execution of an `INSERT` statement.
#[derive(Error, Debug, Diagnostic)]
pub enum InsertError {
    #[error("Field '{field}' specified more than once in INSERT statement for table '{table}'")]
    DuplicateAssignment { table: String, field: String },

    #[error(
        "Field '{field}' is required but was not provided in INSERT statement for table '{table}'"
    )]
    MissingField { table: String, field: String },

    #[error("A record with key '{key}' already exists in table '{table}'")]
    DuplicateKey { table: String, key: String },
}
