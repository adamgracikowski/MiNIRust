use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    DatabaseResult,
    ast::SaveAsStmt,
    core::{Database, DatabaseKey},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `SAVE_AS` command.
///
/// This struct couples the parsed `SaveAsStmt` (the "what") with a
/// mutable reference to the `Database` (the "where") needed to
/// access the command history.
///
/// Note: Although the database reference is mutable, this operation
/// only reads from its history.
pub struct SaveAs<'a, K: DatabaseKey> {
    /// A mutable reference to the database instance.
    pub database: &'a mut Database<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `SAVE_AS` statement,
    /// which contains the target file path.
    pub ast: SaveAsStmt,
}

impl<'a, K: DatabaseKey> Execute for SaveAs<'a, K> {
    /// Executes the `SAVE_AS` command.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * `DatabaseError::Io` - The file cannot be created or written to.
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let file = File::create(&self.ast.path)?;
        let mut writer = BufWriter::new(file);

        for query in self.database.iter_history() {
            writeln!(&mut writer, "{query}")?;
        }

        Ok(ExecutionResult::Success)
    }
}
