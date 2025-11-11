use std::fs;

use crate::{
    DatabaseResult, QueryParser,
    ast::ReadFromStmt,
    core::{Database, DatabaseKey},
    execution::{Execute, ExecutionResult, build_execute_command},
};

/// Represents an executable `READ_FROM` command.
///
/// This struct couples the parsed `ReadFromStmt` (the "what") with a
/// mutable reference to the `Database` (the "where") needed to
/// execute the script's commands.
pub struct ReadFrom<'a, K: DatabaseKey> {
    /// A mutable reference to the database instance, which will be modified
    /// by the commands within the script.
    pub database: &'a mut Database<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `READ_FROM` statement,
    /// which contains the file path.
    pub ast: ReadFromStmt,
}

impl<'a, K: DatabaseKey> Execute for ReadFrom<'a, K> {
    /// Executes the `READ_FROM` command.
    ///
    /// # Errors
    ///
    /// This function will stop and return an error immediately if *any*
    /// operation in the script fails (e.g., file I/O, parsing, validation,
    /// or execution of a sub-command).
    ///
    /// # Returns
    ///
    /// Returns `Ok(ExecutionResult::Messages(log))` containing a detailed log
    /// of the script's execution upon successful completion of all queries.
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let path = &self.ast.path;
        let file_content = fs::read_to_string(path)?;
        let queries = file_content
            .split_inclusive(";")
            .map(|q| q.trim())
            .filter(|q| !q.is_empty());

        let parser = QueryParser;

        let mut log_messages = Vec::new();

        log_messages.push(format!("Reading queries from '{path}'..."));

        for (index, query) in queries.enumerate() {
            log_messages.push(format!("[Query {}] Executing: {query}...", index + 1));

            let ast = parser.parse_query(query)?;

            let mut executable_command = build_execute_command(self.database, ast)?;
            let result = executable_command.execute()?;
            log_messages.push(format!("[Query {}] ...Success: {result:?}", index + 1));
        }

        log_messages.push("Script executed successfully.".to_string());

        Ok(ExecutionResult::Messages(log_messages))
    }
}
