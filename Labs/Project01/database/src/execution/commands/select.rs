use std::collections::{HashMap, HashSet};

use crate::{
    DatabaseResult,
    ast::{Condition, OrderDirection, SelectStmt},
    core::{DatabaseError, DatabaseKey, Record, SelectError, Table},
    execution::{Execute, ExecutionResult},
};

/// Represents an executable `SELECT` command.
///
/// This struct couples the parsed `SelectStmt` (the "what") with a
/// read-only reference to the specific `Table` (the "where") needed to
/// execute the query.
pub struct Select<'a, K: DatabaseKey> {
    /// A read-only reference to the table being queried.
    pub table: &'a Table<K>,
    /// The parsed AST (Abstract Syntax Tree) for the `SELECT` statement.
    pub ast: SelectStmt,
}

impl<'a, K: DatabaseKey> Select<'a, K> {
    /// Recursively evaluates a `WHERE` clause condition tree for a single `Record`.
    ///
    /// This function walks the `Condition` enum (`And`, `Or`, `Comparison`)
    /// and returns `true` if the record matches the filter, or `false` otherwise.
    ///
    /// # Errors
    ///
    /// * `DatabaseError::FieldNotFound` if a field in a `Comparison` does not
    ///   exist in the record.
    /// * `DatabaseError::TypeMismatch` if a comparison is attempted between
    ///   incompatible types (e.g., `String` and `Int`).
    fn evaluate_condition(
        record: &Record,
        condition: &Condition,
        table_name: &str,
    ) -> DatabaseResult<bool> {
        match condition {
            Condition::And { left, right } => {
                Ok(Self::evaluate_condition(record, left, table_name)?
                    && Self::evaluate_condition(record, right, table_name)?)
            }
            Condition::Or { left, right } => {
                Ok(Self::evaluate_condition(record, left, table_name)?
                    || Self::evaluate_condition(record, right, table_name)?)
            }
            Condition::Comparison(comparison) => {
                let record_value = record.fields.get(&comparison.field).ok_or_else(|| {
                    DatabaseError::FieldNotFound {
                        table: table_name.to_string(),
                        field: comparison.field.clone(),
                    }
                })?;

                record_value
                    .compare(&comparison.op, &comparison.value)
                    .map_err(|e| match e {
                        DatabaseError::ComparisonTypeMismatch { expected, found } => {
                            DatabaseError::TypeMismatch {
                                table: table_name.to_string(),
                                field: comparison.field.clone(),
                                expected,
                                found,
                            }
                        }
                        _ => e,
                    })
            }
        }
    }
}

impl<'a, K: DatabaseKey> Execute for Select<'a, K> {
    /// Executes the `SELECT` query.
    ///
    /// This method performs the full query execution pipeline in the standard
    /// SQL logical order:
    ///
    /// 1.  **(Validation):** Checks if all fields in `SELECT` and `ORDER BY` clauses
    ///     exist in the table's schema.
    /// 2.  **(FROM):** Retrieves all records from the table.
    /// 3.  **(WHERE):** Filters the records based on the `where_clause`.
    /// 4.  **(ORDER BY):** Sorts the filtered records.
    /// 5.  **(LIMIT):** Takes the top `N` records.
    /// 6.  **(SELECT):** Projects the final set of records, creating new `Record`
    ///     objects containing only the requested fields.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * `DatabaseError::FieldNotFound` - A field in the `SELECT` or `ORDER BY`
    ///   clause does not exist in the schema.
    /// * An error occurs during `WHERE` clause evaluation (see `evaluate_condition`).
    /// * `SelectError::InvalidLimit` - The `LIMIT` value is negative.
    fn execute(&mut self) -> DatabaseResult<ExecutionResult> {
        let table_name = self.table.name.clone();
        let all_columns = self.table.schema.keys().collect::<HashSet<_>>();
        let not_found_fields = self
            .ast
            .fields
            .iter()
            .filter(|field| !all_columns.contains(field))
            .collect::<Vec<_>>();

        if !not_found_fields.is_empty() {
            return Err(DatabaseError::FieldNotFound {
                table: table_name,
                field: not_found_fields.first().unwrap().to_string(),
            });
        }

        if let Some(order_by) = &self.ast.optional_clauses.order_by
            && !all_columns.contains(&order_by.column)
        {
            return Err(DatabaseError::FieldNotFound {
                table: table_name,
                field: order_by.column.clone(),
            });
        }

        let all_rows = self.table.rows.values().collect::<Vec<_>>();

        let mut filtered_rows: Vec<&Record> = match &self.ast.optional_clauses.where_clause {
            Some(condition) => all_rows.into_iter().try_fold(Vec::new(), |mut acc, row| {
                if Self::evaluate_condition(row, condition, &table_name)? {
                    acc.push(row);
                }
                Ok::<Vec<&Record>, DatabaseError>(acc)
            })?,
            None => all_rows,
        };

        let sorted_rows = match &self.ast.optional_clauses.order_by {
            Some(order_by) => {
                filtered_rows.sort_by_key(|row| row.fields.get(&order_by.column));
                if order_by.direction == OrderDirection::Desc {
                    filtered_rows.reverse();
                }
                filtered_rows
            }
            None => filtered_rows,
        };

        let limited_rows = match &self.ast.optional_clauses.limit {
            Some(limit) if *limit < 0 => {
                return Err(SelectError::InvalidLimit { limit: *limit }.into());
            }
            Some(limit) => sorted_rows.into_iter().take(*limit as usize).collect(),
            None => sorted_rows,
        };

        let projected_rows = limited_rows
            .into_iter()
            .map(|row| {
                let mut fields = HashMap::new();
                for field_name in &self.ast.fields {
                    let value = row.fields.get(field_name).unwrap().clone();
                    fields.insert(field_name.clone(), value);
                }
                Ok(Record { fields })
            })
            .collect::<DatabaseResult<Vec<Record>>>()?;

        Ok(ExecutionResult::Data(projected_rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            Assignment, Comparison, Condition, CreateStmt, Field, InsertStmt, Operator,
            OptionalClauses, OrderBy, OrderDirection, SelectStmt,
        },
        core::{DataType, DataValue},
        execution::Execute,
    };

    mod common {
        use crate::{
            core::{DataType, DataValue, Database},
            execution::commands::{create::Create, insert::Insert},
        };

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
                Field {
                    name: "age".to_string(),
                    data_type: DataType::Int,
                },
                Field {
                    name: "active".to_string(),
                    data_type: DataType::Boolean,
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

            insert_record(
                &mut db,
                1,
                DataValue::String("Alice".to_string()),
                DataValue::Int(30),
                DataValue::Boolean(true),
            );
            insert_record(
                &mut db,
                2,
                DataValue::String("Bob".to_string()),
                DataValue::Int(25),
                DataValue::Boolean(true),
            );
            insert_record(
                &mut db,
                3,
                DataValue::String("Charlie".to_string()),
                DataValue::Int(40),
                DataValue::Boolean(false),
            );

            db
        }

        fn insert_record(
            db: &mut Database<i64>,
            id: i64,
            name: DataValue,
            age: DataValue,
            active: DataValue,
        ) {
            let assignments = vec![
                Assignment {
                    field_name: "id".to_string(),
                    value: DataValue::Int(id),
                },
                Assignment {
                    field_name: "name".to_string(),
                    value: name,
                },
                Assignment {
                    field_name: "age".to_string(),
                    value: age,
                },
                Assignment {
                    field_name: "active".to_string(),
                    value: active,
                },
            ];
            let ast = InsertStmt {
                table_name: "users".to_string(),
                assignments,
                query: "Insert...".to_string(), //unused
            };
            let table = db.tables.get_mut("users").unwrap();
            let mut cmd = Insert { table, ast };
            cmd.execute().unwrap();
        }
    }

    fn get_data_from_result(result: DatabaseResult<ExecutionResult>) -> Vec<Record> {
        match result.unwrap() {
            ExecutionResult::Data(records) => records,
            _ => panic!("Expected ExecutionResult::Data"),
        }
    }

    #[test]
    fn test_execute_select_simple_no_clauses() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let ast = SelectStmt {
            fields: vec!["id".to_string(), "name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses::default(),
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_ok());

        let data = get_data_from_result(result);
        assert_eq!(data.len(), 3);
        assert_eq!(data[0].fields.len(), 2);
        assert!(data[0].fields.contains_key("id"));
        assert!(data[0].fields.contains_key("name"));
    }

    #[test]
    fn test_execute_select_with_where() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let condition = Condition::Comparison(Comparison {
            field: "age".to_string(),
            op: Operator::Gt,
            value: DataValue::Int(25),
        });

        let ast = SelectStmt {
            fields: vec!["name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                where_clause: Some(condition),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_ok());

        let data = get_data_from_result(result);
        assert_eq!(data.len(), 2);
        assert_eq!(
            data[0].fields.get("name"),
            Some(&DataValue::String("Alice".to_string()))
        );
        assert_eq!(
            data[1].fields.get("name"),
            Some(&DataValue::String("Charlie".to_string()))
        );
    }

    #[test]
    fn test_execute_select_with_and() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let condition = Condition::And {
            left: Box::new(Condition::Comparison(Comparison {
                field: "age".to_string(),
                op: Operator::Gt,
                value: DataValue::Int(20),
            })),
            right: Box::new(Condition::Comparison(Comparison {
                field: "active".to_string(),
                op: Operator::Eq,
                value: DataValue::Boolean(true),
            })),
        };

        let ast = SelectStmt {
            fields: vec!["name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                where_clause: Some(condition),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_ok());

        let data = get_data_from_result(result);
        assert_eq!(data.len(), 2);
        assert_eq!(
            data[0].fields.get("name"),
            Some(&DataValue::String("Alice".to_string()))
        );
        assert_eq!(
            data[1].fields.get("name"),
            Some(&DataValue::String("Bob".to_string()))
        );
    }

    #[test]
    fn test_execute_select_with_order_by() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let ast = SelectStmt {
            fields: vec!["name".to_string(), "age".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                order_by: Some(OrderBy {
                    column: "age".to_string(),
                    direction: OrderDirection::Desc,
                }),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_ok());

        let data = get_data_from_result(result);
        assert_eq!(data.len(), 3);
        assert_eq!(
            data[0].fields.get("name"),
            Some(&DataValue::String("Charlie".to_string()))
        );
        assert_eq!(
            data[1].fields.get("name"),
            Some(&DataValue::String("Alice".to_string()))
        );
        assert_eq!(
            data[2].fields.get("name"),
            Some(&DataValue::String("Bob".to_string()))
        );
    }

    #[test]
    fn test_execute_select_with_limit() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let ast = SelectStmt {
            fields: vec!["id".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                limit: Some(2),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_ok());

        let data = get_data_from_result(result);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_execute_select_all_clauses() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let condition = Condition::Comparison(Comparison {
            field: "active".to_string(),
            op: Operator::Eq,
            value: DataValue::Boolean(true),
        });

        let ast = SelectStmt {
            fields: vec!["name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                where_clause: Some(condition),
                order_by: Some(OrderBy {
                    column: "age".to_string(),
                    direction: OrderDirection::Asc,
                }),
                limit: Some(1),
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_ok());

        let data = get_data_from_result(result);
        assert_eq!(data.len(), 1);
        assert_eq!(
            data[0].fields.get("name"),
            Some(&DataValue::String("Bob".to_string()))
        );
    }

    #[test]
    fn test_execute_fail_select_field_not_found() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let ast = SelectStmt {
            fields: vec!["id".to_string(), "email".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses::default(),
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::FieldNotFound { table, field } => {
                assert_eq!(table, "users");
                assert_eq!(field, "email");
            }
            _ => panic!("Expected FieldNotFound error"),
        }
    }

    #[test]
    fn test_execute_fail_orderby_field_not_found() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let ast = SelectStmt {
            fields: vec!["id".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                order_by: Some(OrderBy {
                    column: "salary".to_string(),
                    direction: OrderDirection::Asc,
                }),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::FieldNotFound { table, field } => {
                assert_eq!(table, "users");
                assert_eq!(field, "salary");
            }
            _ => panic!("Expected FieldNotFound error"),
        }
    }

    #[test]
    fn test_execute_fail_where_field_not_found() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let condition = Condition::Comparison(Comparison {
            field: "location".to_string(),
            op: Operator::Eq,
            value: DataValue::String("USA".to_string()),
        });

        let ast = SelectStmt {
            fields: vec!["name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                where_clause: Some(condition),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::FieldNotFound { table, field } => {
                assert_eq!(table, "users");
                assert_eq!(field, "location");
            }
            _ => panic!("Expected FieldNotFound error"),
        }
    }

    #[test]
    fn test_execute_fail_where_type_mismatch() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let condition = Condition::Comparison(Comparison {
            field: "age".to_string(),
            op: Operator::Gt,
            value: DataValue::String("twenty".to_string()),
        });

        let ast = SelectStmt {
            fields: vec!["name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                where_clause: Some(condition),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
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
    }

    #[test]
    fn test_execute_fail_invalid_limit() {
        let mut db = common::setup_db_i64_with_data();
        let table = db.tables.get_mut("users").unwrap();

        let ast = SelectStmt {
            fields: vec!["id".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                limit: Some(-5),
                ..Default::default()
            },
            query: "Select...".to_string(), // unused
        };

        let mut cmd = Select { table, ast };
        let result = cmd.execute();
        assert!(result.is_err());

        match result.err().unwrap() {
            DatabaseError::Select(SelectError::InvalidLimit { limit }) => {
                assert_eq!(limit, -5);
            }
            _ => panic!("Expected InvalidLimit error"),
        }
    }
}
