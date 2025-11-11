use crate::DatabaseResult;

use super::ExecutionResult;

/// Defines a common interface for all executable commands.
pub trait Execute {
    /// Executes the specific logic of the command.
    ///
    /// # Errors
    ///
    /// Returns a `DatabaseError` if any part of the execution fails (e.g.,
    /// a duplicate key error, I/O error, or data validation failure).
    fn execute(&mut self) -> DatabaseResult<ExecutionResult>;
}
