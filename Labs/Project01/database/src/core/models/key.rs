use std::fmt::Debug;
use std::hash::Hash;

use bincode::{Decode, Encode};

use crate::core::{DataType, DataValue};

/// Defines the contract for types that can be used as a primary key in a table.
///
/// This trait combines standard Rust traits (like `Clone`, `Eq`, `Hash`, `Ord`)
/// with traits required for binary serialization (`Encode`, `Decode`) and
/// custom database logic (`key_type`, `from_datavalue`).
pub trait DatabaseKey:
    Clone + Eq + Hash + Ord + Debug + Sized + Encode + Decode<()>
{
    /// Returns the corresponding `DataType` enum variant for this key type.
    ///
    /// This is used for type validation, e.g., ensuring an `i64` key
    /// corresponds to a schema's `DataType::Int`.
    fn key_type() -> DataType;

    /// Attempts to convert a dynamic `DataValue` into a concrete key of type `Self`.
    ///
    /// This is used during operations like `DELETE` or `INSERT` to validate and
    /// convert a value from the AST into the table's specific key type.
    ///
    /// Returns `Some(Self)` if the `DataValue` variant matches the key type,
    /// and `None` otherwise.
    fn from_datavalue(value: &DataValue) -> Option<Self>;
}

/// Implements the `DatabaseKey` contract for `String`.
impl DatabaseKey for String {
    /// The `DataType` equivalent for a `String` key is `DataType::String`.
    fn key_type() -> DataType {
        DataType::String
    }

    /// Converts a `DataValue::String` into an owned `String`.
    /// Returns `None` for any other `DataValue` variant.
    fn from_datavalue(value: &DataValue) -> Option<Self> {
        match value {
            DataValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

/// Implements the `DatabaseKey` contract for `i64`.
impl DatabaseKey for i64 {
    /// The `DataType` equivalent for an `i64` key is `DataType::Int`.
    fn key_type() -> DataType {
        DataType::Int
    }

    /// Converts a `DataValue::Int` into an `i64`.
    /// Returns `None` for any other `DataValue` variant.
    fn from_datavalue(value: &DataValue) -> Option<Self> {
        match value {
            DataValue::Int(i) => Some(*i),
            _ => None,
        }
    }
}