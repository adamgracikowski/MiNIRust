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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Assignment, CreateStmt, Field, InsertStmt},
        core::{DataType, DataValue, Database, DatabaseError, InsertError},
        execution::Execute,
    };

    mod common {
        use crate::execution::commands::create::Create;

        use super::*;

        pub fn setup_db_i64() -> Database<i64> {
            let mut db = Database::<i64>::default();
            let fields = vec![
                Field {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                Field {
                    name: "name".to_string(),
                    data_type: DataType::String,
                },
                Field {
                    name: "age".to_string(),
                    data_type: DataType::Int,
                },
            ];
            let ast = CreateStmt {
                table_name: "users".to_string(),
                key_field: "id".to_string(),
                fields,
                query: "".to_string(),
            };
            let mut cmd = Create {
                database: &mut db,
                ast,
            };
            cmd.execute().expect("Failed to create setup table");
            db
        }
    }

    fn create_valid_ast() -> InsertStmt {
        InsertStmt {
            table_name: "users".to_string(),
            assignments: vec![
                Assignment {
                    field_name: "id".to_string(),
                    value: DataValue::Int(1),
                },
                Assignment {
                    field_name: "name".to_string(),
                    value: DataValue::String("Alice".to_string()),
                },
                Assignment {
                    field_name: "age".to_string(),
                    value: DataValue::Int(30),
                },
            ],
            query: "Insert...".to_string(), // unused
        }
    }

    #[test]
    fn test_execute_insert_success() {
        let mut db = common::setup_db_i64();
        let ast = create_valid_ast();
        let table = db.tables.get_mut("users").unwrap();
        let mut cmd = Insert { table, ast };

        let result = cmd.execute();
        assert!(
            result.is_ok(),
            "Insert failed when it should have succeeded"
        );

        match result.unwrap() {
            ExecutionResult::RowsAffected(count) => assert_eq!(count, 1),
            _ => panic!("Expected RowsAffected(1)"),
        }

        assert_eq!(db.tables["users"].rows.len(), 1);
        let record = db.tables["users"].rows.get(&1).unwrap();
        assert_eq!(
            record.fields.get("name"),
            Some(&DataValue::String("Alice".to_string()))
        );
    }

    #[test]
    fn test_execute_fail_field_not_found() {
        let mut db = common::setup_db_i64();
        let mut ast = create_valid_ast();
        ast.assignments.push(Assignment {
            field_name: "email".to_string(),
            value: DataValue::String("a@b.com".to_string()),
        });

        let table = db.tables.get_mut("users").unwrap();
        let mut cmd = Insert { table, ast };

        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::FieldNotFound { table, field } => {
                assert_eq!(table, "users");
                assert_eq!(field, "email");
            }
            _ => panic!("Expected FieldNotFound error"),
        }
        assert_eq!(db.tables["users"].rows.len(), 0);
    }

    #[test]
    fn test_execute_fail_type_mismatch() {
        let mut db = common::setup_db_i64();
        let mut ast = create_valid_ast();
        ast.assignments[2] = Assignment {
            field_name: "age".to_string(),
            value: DataValue::String("thirty".to_string()),
        };

        let table = db.tables.get_mut("users").unwrap();
        let mut cmd = Insert { table, ast };

        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::TypeMismatch {
                table,
                field,
                expected,
                found,
            } => {
                assert_eq!(table, "users");
                assert_eq!(field, "age");
                assert_eq!(expected, DataType::Int);
                assert_eq!(found, DataType::String);
            }
            _ => panic!("Expected TypeMismatch error"),
        }
        assert_eq!(db.tables["users"].rows.len(), 0);
    }

    #[test]
    fn test_execute_fail_duplicate_assignment() {
        let mut db = common::setup_db_i64();
        let mut ast = create_valid_ast();
        ast.assignments.push(Assignment {
            field_name: "age".to_string(),
            value: DataValue::Int(40),
        });

        let table = db.tables.get_mut("users").unwrap();
        let mut cmd = Insert { table, ast };

        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::Insert(InsertError::DuplicateAssignment { table, field }) => {
                assert_eq!(table, "users");
                assert_eq!(field, "age");
            }
            _ => panic!("Expected DuplicateAssignment error"),
        }
        assert_eq!(db.tables["users"].rows.len(), 0);
    }

    #[test]
    fn test_execute_fail_missing_field() {
        let mut db = common::setup_db_i64();
        let mut ast = create_valid_ast();
        ast.assignments.pop();

        let table = db.tables.get_mut("users").unwrap();
        let mut cmd = Insert { table, ast };

        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::Insert(InsertError::MissingField { table, field }) => {
                assert_eq!(table, "users");
                assert_eq!(field, "age");
            }
            _ => panic!("Expected MissingField error"),
        }
        assert_eq!(db.tables["users"].rows.len(), 0);
    }

    #[test]
    fn test_execute_fail_duplicate_key() {
        let mut db = common::setup_db_i64();

        {
            let ast1 = create_valid_ast();
            let table1 = db.tables.get_mut("users").unwrap();
            let mut cmd1 = Insert {
                table: table1,
                ast: ast1,
            };

            let result1 = cmd1.execute();
            assert!(result1.is_ok(), "First insert failed");
        }

        assert_eq!(db.tables["users"].rows.len(), 1);

        {
            let ast2 = create_valid_ast();
            let table2 = db.tables.get_mut("users").unwrap();
            let mut cmd2 = Insert {
                table: table2,
                ast: ast2,
            };

            let result2 = cmd2.execute();
            assert!(
                result2.is_err(),
                "Second insert succeeded when it should have failed"
            );

            match result2.err().unwrap() {
                DatabaseError::Insert(InsertError::DuplicateKey { table, key }) => {
                    assert_eq!(table, "users");
                    assert_eq!(key, "1");
                }
                _ => panic!("Expected DuplicateKey error"),
            }
        }

        assert_eq!(db.tables["users"].rows.len(), 1);
    }
}
