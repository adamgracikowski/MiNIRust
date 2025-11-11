use crate::core::DataValue;

/// Represents a `DELETE FROM` SQL statement.
///
/// This struct holds the information required to delete a single record
/// from a table based on its primary key value.
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStmt {
    /// The name of the table from which to delete a record.
    pub table_name: String,
    /// The primary key value of the record to be deleted.
    pub key_value: DataValue,
    /// The raw, original query string that was parsed to create this statement.
    pub query: String,
}