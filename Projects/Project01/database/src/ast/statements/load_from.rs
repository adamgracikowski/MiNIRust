/// Represents a `LOAD_FROM` statement.
///
/// This struct holds the file path from which a binary database snapshot
/// should be loaded, replacing the current database state.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadFromStmt {
    /// The source file path for the database snapshot.
    pub path: String,
}
