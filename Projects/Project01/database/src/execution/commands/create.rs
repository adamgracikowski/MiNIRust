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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{CreateStmt, Field},
        core::CreateError,
    };

    fn mock_users_ast() -> CreateStmt {
        CreateStmt {
            table_name: "users".to_string(),
            key_field: "id".to_string(),
            fields: vec![
                Field {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                Field {
                    name: "name".to_string(),
                    data_type: DataType::String,
                },
            ],
            query: "CREATE...".to_string(), // unused
        }
    }

    #[test]
    fn test_execute_create_success() {
        let mut db = Database::<i64>::default();
        let ast = mock_users_ast();
        let mut cmd = Create {
            database: &mut db,
            ast,
        };

        let result = cmd.execute();
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), ExecutionResult::Success));

        assert!(db.tables.contains_key("users"));
        let table = &db.tables["users"];
        assert_eq!(table.name, "users");
        assert_eq!(table.key_field, "id");
        assert_eq!(table.schema.len(), 2);
        assert_eq!(table.schema.get("name"), Some(&DataType::String));
    }

    #[test]
    fn test_execute_fail_duplicate_column() {
        let mut db = Database::<i64>::default();
        let mut ast = mock_users_ast();

        ast.fields.push(Field {
            name: "id".to_string(),
            data_type: DataType::Int,
        });

        let mut cmd = Create {
            database: &mut db,
            ast,
        };
        let result = cmd.execute();

        assert!(result.is_err());
        match result.err().unwrap() {
            DatabaseError::DuplicateColumn { table, column } => {
                assert_eq!(table, "users");
                assert_eq!(column, "id");
            }
            _ => panic!("Received invalid error type"),
        }
        assert!(db.tables.is_empty());
    }

    #[test]
    fn test_execute_fail_key_field_not_found() {
        let mut db = Database::<i64>::default();
        let mut ast = mock_users_ast();

        ast.key_field = "uuid".to_string();

        let mut cmd = Create {
            database: &mut db,
            ast,
        };
        let result = cmd.execute();

        assert!(result.is_err());
        match result.err().unwrap() {
            DatabaseError::FieldNotFound { table, field } => {
                assert_eq!(table, "users");
                assert_eq!(field, "uuid");
            }
            _ => panic!("Received invalid error type"),
        }
        assert!(db.tables.is_empty());
    }

    #[test]
    fn test_execute_fail_key_type_mismatch() {
        let mut db = Database::<String>::default();
        let ast = mock_users_ast();
        let mut cmd = Create {
            database: &mut db,
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
                assert_eq!(expected, DataType::String);
                assert_eq!(found, DataType::Int);
            }
            _ => panic!("Received invalid error type"),
        }
        assert!(db.tables.is_empty());
    }

    #[test]
    fn test_execute_fail_table_already_exists() {
        let mut db = Database::<i64>::default();
        let ast1 = mock_users_ast();
        let mut cmd1 = Create {
            database: &mut db,
            ast: ast1,
        };

        assert!(cmd1.execute().is_ok());
        assert_eq!(db.tables.len(), 1);

        let ast2 = mock_users_ast();
        let mut cmd2 = Create {
            database: &mut db,
            ast: ast2,
        };
        let result = cmd2.execute();

        assert!(result.is_err());
        match result.err().unwrap() {
            DatabaseError::Create(CreateError::TableAlreadyExists { name }) => {
                assert_eq!(name, "users");
            }
            _ => panic!("Received invalid error type"),
        }
        assert_eq!(db.tables.len(), 1);
    }
}
