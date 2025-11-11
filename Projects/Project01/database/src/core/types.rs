use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use bincode::{Decode, Encode};

use crate::{
    DatabaseResult,
    ast::{AstError, Operator},
};

use super::errors::DatabaseError;

/// Represents the set of all possible data types for a column in a table's schema.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Encode, Decode)]
pub enum DataType {
    /// A boolean value (`true` or `false`).
    Boolean,
    /// A UTF-8 encoded string.
    String,
    /// A 64-bit signed integer.
    Int,
    /// A 64-bit floating-point number.
    Float,
}

/// Enables parsing a `DataType` from a string slice (e.g., from the parser).
impl TryFrom<&str> for DataType {
    type Error = DatabaseError;

    /// Attempts to parse a string slice (e.g., "INT", "STRING") into a `DataType`.
    ///
    /// # Errors
    /// Returns `DatabaseError::UnknownDataType` if the string is not a valid type.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data_type = match value {
            "INT" => Self::Int,
            "FLOAT" => Self::Float,
            "STRING" => Self::String,
            "BOOLEAN" => Self::Boolean,
            other => {
                return Err(DatabaseError::UnknownDataType {
                    data_type: other.to_string(),
                });
            }
        };
        Ok(data_type)
    }
}

/// Represents a single, concrete value of any type supported by the database.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum DataValue {
    /// A boolean value (`true` or `false`).
    Boolean(bool),
    /// A UTF-8 encoded string.
    String(String),
    /// A 64-bit signed integer.
    Int(i64),
    /// A 64-bit floating-point number.
    Float(f64),
}

impl DataValue {
    /// Returns the corresponding `DataType` variant for this `DataValue`.
    pub fn get_type(&self) -> DataType {
        match self {
            Self::Int(_) => DataType::Int,
            Self::Float(_) => DataType::Float,
            Self::String(_) => DataType::String,
            Self::Boolean(_) => DataType::Boolean,
        }
    }

    /// Compares this `DataValue` (left) against another `DataValue` (right)
    /// using the specified `Operator`.
    ///
    /// # Errors
    ///
    /// * `DatabaseError::ComparisonTypeMismatch` if the two values are of different types.
    /// * `AstError::InvalidOperatorForType` if the operator is not valid for the type
    ///   (e.g., `>` on a `Boolean`).
    pub fn compare(&self, op: &Operator, right: &Self) -> DatabaseResult<bool> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(op.evaluate(l, r)),
            (Self::Float(l), Self::Float(r)) => Ok(op.evaluate(l, r)),
            (Self::String(l), Self::String(r)) => Ok(op.evaluate(l, r)),

            (Self::Boolean(l), Self::Boolean(r)) => match op {
                Operator::Eq => Ok(l == r),
                Operator::NotEq => Ok(l != r),
                _ => Err(AstError::InvalidOperatorForType {
                    operator: op.clone(),
                    dtype: DataType::Boolean,
                }
                .into()),
            },

            (l, r) => Err(DatabaseError::ComparisonTypeMismatch {
                expected: l.get_type(),
                found: r.get_type(),
            }),
        }
    }
}

/// Provides a user-friendly, SQL-like string representation of the `DataValue`.
impl Display for DataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(v) => write!(f, "{v}"),
            Self::String(v) => {
                let escaped = v.replace('"', "\\\"");
                write!(f, "\"{escaped}\"")
            }
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
        }
    }
}

/// Manually implements `Eq`.
///
/// This is required because `f64` does not implement `Eq`.
impl Eq for DataValue {}

/// Implements partial ordering for `DataValue`.
///
/// This implementation only provides an ordering for values of the *same* type.
/// It returns `None` if the types are different (e.g., Int vs. String).
impl PartialOrd for DataValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implements total ordering for `DataValue`.
///
/// This is required for sorting (`ORDER BY`). It establishes a fixed, arbitrary
/// order between different types and provides a total order for `f64`
/// by handling `NaN` values.
///
/// The defined cross-type order is: `Boolean < Int < Float < String`.
/// `NaN` is treated as the smallest possible `Float` value.
impl Ord for DataValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Boolean(l), Self::Boolean(r)) => l.cmp(r),
            (Self::Int(l), Self::Int(r)) => l.cmp(r),
            (Self::String(l), Self::String(r)) => l.cmp(r),
            (Self::Float(l), Self::Float(r)) => {
                if l.is_nan() && r.is_nan() {
                    Ordering::Equal
                } else if l.is_nan() {
                    Ordering::Less // NaN is considered smallest
                } else if r.is_nan() {
                    Ordering::Greater
                } else {
                    l.partial_cmp(r).unwrap()
                }
            }

            // Order: Boolean < Int < Float < String
            (Self::Boolean(_), _) => Ordering::Less,
            (_, Self::Boolean(_)) => Ordering::Greater,

            (Self::Int(_), _) => Ordering::Less,
            (_, Self::Int(_)) => Ordering::Greater,

            (Self::Float(_), _) => Ordering::Less,
            (_, Self::Float(_)) => Ordering::Greater,
        }
    }
}
