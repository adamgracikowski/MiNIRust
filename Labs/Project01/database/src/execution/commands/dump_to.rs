use bincode::{config, encode_into_std_write};
use std::{fs::File, io::BufWriter};

use crate::{
    DatabaseResult,
    ast::DumpToStmt,
    core::{Database, DatabaseKey},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `DUMP_TO` command.
///
/// This struct couples the parsed `DumpToStmt` (the "what") with a
/// read-only reference to the `Database` (the "where") needed to
/// perform the binary serialization.
pub struct DumpTo<'a, K: DatabaseKey> {
    /// A read-only reference to the database instance that will be serialized.
    pub database: &'a Database<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `DUMP_TO` statement.
    pub ast: DumpToStmt,
}

impl<'a, K: DatabaseKey> Execute for DumpTo<'a, K> {
    /// Executes the `DUMP_TO` command.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * `DatabaseError::Io` - The file cannot be created or written to.
    /// * `DatabaseError::Encode` - `bincode` fails to serialize the database.
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let file = File::create(&self.ast.path)?;
        let mut writer = BufWriter::new(file);

        let config = config::standard();

        match encode_into_std_write(self.database, &mut writer, config) {
            Ok(_) => Ok(ExecutionResult::Success),
            Err(e) => Err(e.into()),
        }
    }
}
