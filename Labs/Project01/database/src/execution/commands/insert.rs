use std::collections::HashMap;

use crate::{
    DatabaseResult,
    ast::{Assignment, InsertStmt},
    core::{DatabaseError, DatabaseKey, InsertError, Record, Table},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `INSERT INTO` command.
///
/// This struct couples the parsed `InsertStmt` (the "what") with a
/// mutable reference to the specific `Table` (the "where") needed to
/// execute the insertion.
pub struct Insert<'a, K: DatabaseKey> {
    /// A mutable reference to the table where the record will be inserted.
    pub table: &'a mut Table<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `INSERT` statement.
    pub ast: InsertStmt,
}

impl<'a, K: DatabaseKey> Execute for Insert<'a, K> {
    /// Executes the `INSERT INTO` command.
    ///
    /// # Errors
    ///
    /// Returns an error if any validation step fails:
    /// * `DatabaseError::FieldNotFound`
    /// * `DatabaseError::TypeMismatch`
    /// * `InsertError::DuplicateAssignment`
    /// * `InsertError::MissingField`
    /// * `InsertError::DuplicateKey`
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let table_name = self.table.name.clone();
        let mut record_fields = HashMap::with_capacity(self.table.schema.len());

        for Assignment { field_name, value } in &self.ast.assignments {
            let expected_type =
                self.table
                    .schema
                    .get(field_name)
                    .ok_or_else(|| DatabaseError::FieldNotFound {
                        table: table_name.clone(),
                        field: field_name.clone(),
                    })?;

            let actual_type = value.get_type();
            if *expected_type != actual_type {
                return Err(DatabaseError::TypeMismatch {
                    table: table_name.clone(),
                    field: field_name.clone(),
                    expected: *expected_type,
                    found: actual_type,
                });
            }

            if record_fields
                .insert(field_name.clone(), value.clone())
                .is_some()
            {
                return Err(InsertError::DuplicateAssignment {
                    table: table_name,
                    field: field_name.to_string(),
                }
                .into());
            }
        }

        if record_fields.len() != self.table.schema.len() {
            let missing_field = self
                .table
                .schema
                .keys()
                .find(|schema_field| !record_fields.contains_key(*schema_field))
                .unwrap();

            return Err(InsertError::MissingField {
                table: table_name,
                field: missing_field.clone(),
            }
            .into());
        }

        let key_datavalue = record_fields.get(&self.table.key_field).unwrap();
        let key = K::from_datavalue(key_datavalue).unwrap();

        if self.table.rows.contains_key(&key) {
            return Err(InsertError::DuplicateKey {
                table: table_name,
                key: format!("{key:?}"),
            }
            .into());
        }

        let record = Record {
            fields: record_fields,
        };
        self.table.rows.insert(key, record);

        Ok(ExecutionResult::RowsAffected(1))
    }
}
