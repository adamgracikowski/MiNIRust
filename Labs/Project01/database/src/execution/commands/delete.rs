use crate::{
    DatabaseResult,
    ast::DeleteStmt,
    core::{DatabaseError, DatabaseKey, Table},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `DELETE FROM` command.
///
/// This struct couples the parsed `DeleteStmt` (the "what") with a
/// mutable reference to the specific `Table` (the "where") needed to
/// execute the deletion.
pub struct Delete<'a, K: DatabaseKey> {
    /// A mutable reference to the table from which the record will be deleted.
    pub table: &'a mut Table<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `DELETE` statement.
    pub ast: DeleteStmt,
}

impl<'a, K: DatabaseKey> Execute for Delete<'a, K> {
    /// Executes the `DELETE FROM` command.
    ///
    /// # Errors
    ///
    /// Returns `DatabaseError::TypeMismatch` if the provided key value's type
    /// does not match the table's primary key type.
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let key_to_delete =
            K::from_datavalue(&self.ast.key_value).ok_or_else(|| DatabaseError::TypeMismatch {
                table: self.table.name.clone(),
                field: self.table.key_field.clone(),
                expected: K::key_type(),
                found: self.ast.key_value.get_type(),
            })?;

        let removed_record = self.table.rows.remove(&key_to_delete);

        match removed_record {
            Some(_) => Ok(ExecutionResult::RowsAffected(1)),
            None => Ok(ExecutionResult::RowsAffected(0)),
        }
    }
}
