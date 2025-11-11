use crate::{ast::AstError, core::DataValue};

/// Represents an atomic comparison in a `WHERE` clause.
///
/// e.g., `age > 21` or `name = "John"`
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    /// The name of the column (field) on the left side of the comparison.
    pub field: String,
    /// The comparison operator (e.g., `=`, `>`, `!=`).
    pub op: Operator,
    /// The literal value on the right side of the comparison.
    pub value: DataValue,
}

/// Represents a comparison operator (e.g., `=`, `!=`, `<`).
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// Equals (`=`)
    Eq,
    /// Not Equals (`!=`)
    NotEq,
    /// Less Than (`<`)
    Lt,
    /// Greater Than (`>`)
    Gt,
    /// Less Than or Equals (`<=`)
    LtEq,
    /// Greater Than or Equals (`>=`)
    GtEq,
}

impl Operator {
    /// Evaluates the operator on two generic values.
    ///
    /// This method works for any type `T` that implements `PartialEq` (for `==`, `!=`)
    /// and `PartialOrd` (for `<`, `>`, `<=`, `>=`).
    ///
    /// # Arguments
    /// * `left` - The left-hand side value of the comparison.
    /// * `right` - The right-hand side value of the comparison.
    ///
    /// # Returns
    /// `true` if the comparison is successful, `false` otherwise.
    pub fn evaluate<T: PartialEq + PartialOrd>(&self, left: &T, right: &T) -> bool {
        match self {
            Self::Eq => left == right,
            Self::NotEq => left != right,
            Self::Lt => left < right,
            Self::Gt => left > right,
            Self::LtEq => left <= right,
            Self::GtEq => left >= right,
        }
    }
}

/// Enables parsing an `Operator` from a raw string slice.
///
/// This is primarily used by the parser to convert operator tokens
/// (like `"="` or `">"`) into the corresponding enum variant.
impl TryFrom<&str> for Operator {
    type Error = AstError;

    /// Attempts to parse a string slice (e.g., `"="`, `">="`) into an `Operator`.
    ///
    /// # Errors
    ///
    /// Returns `AstError::UnknownOperator` if the string does not match one
    /// of the known operators.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let operator = match value {
            "=" => Self::Eq,
            "!=" => Self::NotEq,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "<=" => Self::LtEq,
            ">=" => Self::GtEq,
            // Handle any unrecognized operator string
            _ => {
                return Err(AstError::UnknownOperator {
                    operator: value.to_string(),
                });
            }
        };
        Ok(operator)
    }
}
