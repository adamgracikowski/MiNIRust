use crate::ast::expressions::Field;

/// Represents a `CREATE TABLE` SQL statement.
///
/// This struct holds all the parsed information required to create a new table,
/// including its name, the designated primary key, and the list of all its fields.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateStmt {
    /// The name of the table to be created.
    pub table_name: String,
    /// The name of the field designated as the primary key for this table.
    pub key_field: String,
    /// A vector of `Field` definitions specifying the schema (all columns and their types).
    pub fields: Vec<Field>,
    /// The raw, original query string that was parsed to create this statement.
    pub query: String,
}