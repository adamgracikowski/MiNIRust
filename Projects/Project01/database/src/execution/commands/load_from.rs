use std::{fs::File, io::BufReader};

use bincode::{config, decode_from_std_read};

use crate::{
    DatabaseResult,
    ast::LoadFromStmt,
    core::{Database, DatabaseKey},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `LOAD_FROM` command.
///
/// This struct couples the parsed `LoadFromStmt` (the "what") with a
/// mutable reference to the `Database` (the "where") needed to
/// perform the deserialization and replace the database state.
pub struct LoadFrom<'a, K: DatabaseKey> {
    /// A mutable reference to the database instance that will be
    /// replaced by the loaded data.
    pub database: &'a mut Database<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `LOAD_FROM` statement.
    pub ast: LoadFromStmt,
}

impl<'a, K: DatabaseKey> Execute for LoadFrom<'a, K> {
    /// Executes the `LOAD_FROM` command.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * `DatabaseError::Io` - The file cannot be opened or read.
    /// * `DatabaseError::Decode` - `bincode` fails to deserialize the database,
    ///   (e.g., due to corrupt data or a type mismatch).
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let file = File::open(&self.ast.path)?;
        let mut reader = BufReader::new(file);
        let config = config::standard();

        let loaded: Database<K> = decode_from_std_read(&mut reader, config)?;

        *self.database = loaded;
        Ok(ExecutionResult::Success)
    }
}
