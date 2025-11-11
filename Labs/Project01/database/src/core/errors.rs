mod create_error;
mod insert_error;
mod select_error;

pub use create_error::CreateError;
pub use insert_error::InsertError;
pub use select_error::SelectError;

use std::io;

use bincode::error::{DecodeError, EncodeError};
use miette::Diagnostic;
use thiserror::Error;

use crate::{ast::AstError, core::DataType, parsing::ParsingError};

/// A specialized `Result` type for all database operations.
///
/// This type alias uses the crate's umbrella error type, `DatabaseError`.
pub type DatabaseResult<T> = std::result::Result<T, DatabaseError>;

/// The primary error enum for all database operations.
///
/// This enum acts as an "umbrella" error type, consolidating errors from
/// I/O, serialization, parsing, AST building, and command execution
/// into a single, unified type.
#[derive(Error, Debug, Diagnostic)]
pub enum DatabaseError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Encode(#[from] EncodeError),

    #[error(transparent)]
    Decode(#[from] DecodeError),

    #[error(transparent)]
    Ast(#[from] AstError),

    #[error(transparent)]
    Parse(#[from] Box<ParsingError>),

    #[error(transparent)]
    Insert(#[from] InsertError),

    #[error(transparent)]
    Create(#[from] CreateError),

    #[error(transparent)]
    Select(#[from] SelectError),

    #[error("Table '{name}' not found")]
    TableNotFound { name: String },

    #[error("Key field '{field}' not found in schema for table '{table}'")]
    FieldNotFound { table: String, field: String },

    #[error(
        "Type mismatch for field '{field}' in table '{table}': expected {expected:?}, found {found:?}"
    )]
    TypeMismatch {
        table: String,
        field: String,
        expected: DataType,
        found: DataType,
    },

    #[error("Duplicate column name '{column}' in table definition '{table}'")]
    DuplicateColumn { table: String, column: String },

    #[error("Type mismatch in comparison: expected {expected:?}, found {found:?}")]
    ComparisonTypeMismatch { expected: DataType, found: DataType },

    #[error("Unknown data type: {data_type}")]
    UnknownDataType { data_type: String },
}
