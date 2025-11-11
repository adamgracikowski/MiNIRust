use crate::ast::expressions::Assignment;

/// Represents an `INSERT INTO` SQL statement.
///
/// This struct holds all the information required to insert a new record
/// into a table, including the target table and a list of all
/// column assignments.
#[derive(Debug, Clone, PartialEq)]
pub struct InsertStmt {
    /// The name of the table to insert a new record into.
    pub table_name: String,
    /// A vector of `Assignment` structs (e.g., `name = "John"`)
    /// specifying the values for the new record.
    pub assignments: Vec<Assignment>,
    /// The raw, original query string that was parsed to create this statement.
    pub query: String,
}
