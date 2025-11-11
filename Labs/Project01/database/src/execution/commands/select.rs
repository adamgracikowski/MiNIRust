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
        &self,
        record: &Record,
        condition: &Condition,
        table_name: &str,
    ) -> DatabaseResult<bool> {
        match condition {
            Condition::And { left, right } => Ok(self
                .evaluate_condition(record, left, table_name)?
                && self.evaluate_condition(record, right, table_name)?),
            Condition::Or { left, right } => Ok(self
                .evaluate_condition(record, left, table_name)?
                || self.evaluate_condition(record, right, table_name)?),
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
                if self.evaluate_condition(row, condition, &table_name)? {
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