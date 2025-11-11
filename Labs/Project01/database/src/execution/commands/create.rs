use std::collections::HashMap;

use crate::{
    DatabaseResult,
    ast::CreateStmt,
    core::{DataType, Database, DatabaseError, DatabaseKey, Table},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `CREATE TABLE` command.
///
/// This struct couples the parsed `CreateStmt` (the "what") with the
/// mutable reference to the `Database` (the "where") needed to
/// execute the operation.
pub struct Create<'a, K: DatabaseKey> {
    /// A mutable reference to the database instance where the table will be created.
    pub database: &'a mut Database<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `CREATE` statement.
    pub ast: CreateStmt,
}

impl<'a, K: DatabaseKey> Execute for Create<'a, K> {
    /// Executes the `CREATE TABLE` command.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * `DatabaseError::DuplicateColumn` - A column name is repeated in the `FIELDS` list.
    /// * `DatabaseError::FieldNotFound` - The specified `key_field` does not exist.
    /// * `DatabaseError::TypeMismatch` - The `key_field`'s type doesn't match `K`.
    /// * `CreateError::TableAlreadyExists` - A table with the same name already exists.
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let mut schema: HashMap<String, DataType> = HashMap::with_capacity(self.ast.fields.len());
        for field_def in self.ast.fields.iter() {
            if schema
                .insert(field_def.name.clone(), field_def.data_type)
                .is_some()
            {
                return Err(DatabaseError::DuplicateColumn {
                    table: self.ast.table_name.clone(),
                    column: field_def.name.clone(),
                });
            }
        }

        let table = Table::new(&self.ast.table_name, &self.ast.key_field, schema)?;

        self.database.add_table(table)?;

        Ok(ExecutionResult::Success)
    }
}
