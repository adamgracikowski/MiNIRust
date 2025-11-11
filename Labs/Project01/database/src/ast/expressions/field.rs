use crate::core::DataType;

/// Represents a field definition within a table's schema.
///
/// e.g., `name: STRING` or `id: INT`
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The name of the column (field).
    pub name: String,
    /// The data type (e.g., `INT`, `STRING`) associated with this field.
    pub data_type: DataType,
}
