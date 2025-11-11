//! The `core` module, defining the fundamental data structures for the database.
//!
//! This module contains the database's primary models (like [Database], [Table], [Record]),
//! error types ([DatabaseError]), and data types ([DataType], [DataValue]).

mod errors;
mod models;
mod types;

pub use errors::{DatabaseError, DatabaseResult, InsertError, SelectError};
pub use models::{Database, DatabaseKey, DatabaseType, Record, Table};
pub use types::{DataType, DataValue};
