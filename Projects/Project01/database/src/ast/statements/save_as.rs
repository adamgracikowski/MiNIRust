/// Represents a `SAVE_AS` statement.
///
/// This struct holds the file path where the database's command history
/// should be saved as a SQL script.
#[derive(Debug, Clone, PartialEq)]
pub struct SaveAsStmt {
    /// The target file path for the SQL history script.
    pub path: String,
}
