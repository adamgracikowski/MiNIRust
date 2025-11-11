/// Represents a `DUMP_TO` statement.
///
/// This struct holds the file path where the database's binary snapshot
/// should be saved.
#[derive(Debug, Clone, PartialEq)]
pub struct DumpToStmt {
    /// The target file path for the database dump.
    pub path: String,
}
