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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{CreateStmt, DeleteStmt, Field},
        core::{DataType, DataValue, Database, Record},
        execution::{Execute, commands::create::Create},
    };
    use std::collections::HashMap;

    mod common {
        use super::*;

        pub fn setup_db_i64_with_data() -> Database<i64> {
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
            ];
            let ast = CreateStmt {
                table_name: "users".to_string(),
                key_field: "id".to_string(),
                fields,
                query: "Create...".to_string(), // unused
            };
            let mut cmd = Create {
                database: &mut db,
                ast,
            };
            cmd.execute().unwrap();

            let mut fields = HashMap::new();
            fields.insert("id".to_string(), DataValue::Int(10));
            fields.insert("name".to_string(), DataValue::String("Alice".to_string()));
            let record = Record { fields };
            db.tables.get_mut("users").unwrap().rows.insert(10, record);

            db
        }

        pub fn setup_db_string_with_data() -> Database<String> {
            let mut db = Database::<String>::default();

            let fields = vec![Field {
                name: "sku".to_string(),
                data_type: DataType::String,
            }];
            let ast = CreateStmt {
                table_name: "products".to_string(),
                key_field: "sku".to_string(),
                fields,
                query: "Create...".to_string(),
            };
            let mut cmd = Create {
                database: &mut db,
                ast,
            };
            cmd.execute().unwrap();

            let mut fields = HashMap::new();
            fields.insert("sku".to_string(), DataValue::String("A123".to_string()));
            let record = Record { fields };
            db.tables
                .get_mut("products")
                .unwrap()
                .rows
                .insert("A123".to_string(), record);

            db
        }
    }

    #[test]
    fn test_execute_delete_success_i64_key() {
        let mut db = common::setup_db_i64_with_data();
        assert_eq!(db.tables["users"].rows.len(), 1);

        let ast = DeleteStmt {
            table_name: "users".to_string(),
            key_value: DataValue::Int(10),
            query: "Delete...".to_string(), // unused
        };
        let table_ref = db.tables.get_mut("users").unwrap();
        let mut cmd = Delete {
            table: table_ref,
            ast,
        };

        let result = cmd.execute();
        assert!(result.is_ok());

        match result.unwrap() {
            ExecutionResult::RowsAffected(count) => assert_eq!(count, 1),
            _ => panic!("Expected RowsAffected(1)"),
        }
        assert_eq!(db.tables["users"].rows.len(), 0);
    }

    #[test]
    fn test_execute_delete_key_not_found_string_key() {
        let mut db = common::setup_db_string_with_data();
        assert_eq!(db.tables["products"].rows.len(), 1);

        let ast = DeleteStmt {
            table_name: "products".to_string(),
            key_value: DataValue::String("B456".to_string()),
            query: "Delete...".to_string(),
        };
        let table_ref = db.tables.get_mut("products").unwrap();
        let mut cmd = Delete {
            table: table_ref,
            ast,
        };

        let result = cmd.execute();
        assert!(result.is_ok());

        match result.unwrap() {
            ExecutionResult::RowsAffected(count) => assert_eq!(count, 0),
            _ => panic!("Expected RowsAffected(0)"),
        }
        assert_eq!(db.tables["products"].rows.len(), 1);
    }

    #[test]
    fn test_execute_fail_key_type_mismatch() {
        let mut db = common::setup_db_i64_with_data();
        assert_eq!(db.tables["users"].rows.len(), 1);

        let ast = DeleteStmt {
            table_name: "users".to_string(),
            key_value: DataValue::String("10".to_string()),
            query: "Delete...".to_string(),
        };
        let table_ref = db.tables.get_mut("users").unwrap();
        let mut cmd = Delete {
            table: table_ref,
            ast,
        };

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
                assert_eq!(field, "id");
                assert_eq!(expected, DataType::Int);
                assert_eq!(found, DataType::String);
            }
            _ => panic!("Expected błędu TypeMismatch"),
        }
        assert_eq!(db.tables["users"].rows.len(), 1);
    }
}
