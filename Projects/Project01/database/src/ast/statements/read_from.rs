/// Represents a `READ_FROM` statement.
///
/// This struct holds the file path of a script containing SQL commands
/// to be executed.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadFromStmt {
    /// The source file path for the SQL script.
    pub path: String,
}
