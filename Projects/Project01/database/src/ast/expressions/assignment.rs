use crate::core::DataValue;

/// Represents a single field assignment in an `INSERT` statement.
///
/// e.g., `name = "John"` or `age = 21`
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    /// The name of the column (field) being assigned a value.
    pub field_name: String,
    /// The specific value being assigned to the field.
    pub value: DataValue,
}
