use pest::{Parser, iterators::Pair};

use super::ParsingError;

use crate::{
    DatabaseResult,
    ast::{
        Assignment, Command, Comparison, Condition, CreateStmt, DeleteStmt, DumpToStmt, Field,
        InsertStmt, LoadFromStmt, Operator, OptionalClauses, OrderBy, OrderDirection, ReadFromStmt,
        SaveAsStmt, SelectStmt,
    },
    core::{DataType, DataValue, DatabaseError},
};

/// The main parser for the database's SQL-like query language.
#[derive(pest_derive::Parser)]
#[grammar = "parsing/query_grammar.pest"]
pub struct QueryParser;

impl QueryParser {
    /// Parses a raw query string into a complete AST `Command`.
    ///
    /// This is the main public entry point for the parser.
    ///
    /// # Arguments
    /// * `input` - The raw SQL-like query string to parse.
    ///
    /// # Returns
    /// A `DatabaseResult` containing the structured `Command` on success.
    ///
    /// # Errors
    /// * `DatabaseError::Parse` if the input string violates the grammar (syntax error).
    /// * `DatabaseError::Ast` or other variants if the AST building fails (e.g., conversion error).
    pub fn parse_query(&self, input: &str) -> DatabaseResult<Command> {
        let pairs = QueryParser::parse(Rule::query, input)
            .map_err(|e| Box::new(ParsingError::from(Box::new(e))))?;
        let query = pairs.into_iter().next().unwrap();
        let query_raw = query.as_span().as_str();
        let command = query.into_inner().next().unwrap();
        self.build_command(command, query_raw)
    }

    /// Builds a specific `Command` from its corresponding `Pair`.
    ///
    /// # Arguments
    /// * `pair` - The `Pair` representing the specific command (e.g., `create_stmt`).
    /// * `query` - The full, raw query string, preserved for history/logging.
    fn build_command(&self, pair: Pair<'_, Rule>, query: &str) -> DatabaseResult<Command> {
        let pair_rule = pair.as_rule();
        let command = match &pair_rule {
            Rule::create_stmt => Command::Create(self.build_create_stmt(pair, query)?),
            Rule::insert_stmt => Command::Insert(self.build_insert_stmt(pair, query)?),
            Rule::delete_stmt => Command::Delete(self.build_delete_stmt(pair, query)?),
            Rule::select_stmt => Command::Select(self.build_select_stmt(pair, query)?),
            Rule::save_as_stmt => {
                let path = self.build_file_path(pair);
                Command::SaveAs(SaveAsStmt { path })
            }
            Rule::read_from_stmt => {
                let path = self.build_file_path(pair);
                Command::ReadFrom(ReadFromStmt { path })
            }
            Rule::dump_to_stmt => {
                let path = self.build_file_path(pair);
                Command::DumpTo(DumpToStmt { path })
            }
            Rule::load_from_stmt => {
                let path = self.build_file_path(pair);
                Command::LoadFrom(LoadFromStmt { path })
            }
            rule => {
                return Err(DatabaseError::from(Box::new(
                    ParsingError::UnexpectedRule { rule: *rule },
                )));
            }
        };
        Ok(command)
    }

    /// Extracts a file path from a file-related statement.
    fn build_file_path(&self, pair: Pair<'_, Rule>) -> String {
        let file_path = pair.into_inner().next().unwrap();
        let string_literal = file_path.into_inner().next().unwrap();
        self.build_string_literal(string_literal)
    }

    /// Converts inner literal (int, float, string, or bool)
    /// and returns the appropriate `DataValue` variant.
    ///
    /// # Errors
    /// Returns a `DatabaseError` if parsing the literal fails (e.g., invalid int).
    fn build_value(&self, pair: Pair<'_, Rule>) -> DatabaseResult<DataValue> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::int_literal => {
                let literal = inner
                    .as_str()
                    .parse::<i64>()
                    .map_err(ParsingError::from)
                    .map_err(Box::new)?;
                Ok(DataValue::Int(literal))
            }
            Rule::float_literal => {
                let literal = inner
                    .as_str()
                    .parse::<f64>()
                    .map_err(ParsingError::from)
                    .map_err(Box::new)?;
                Ok(DataValue::Float(literal))
            }
            Rule::string_literal => Ok(DataValue::String(self.build_string_literal(inner))),
            Rule::bool_literal => {
                let literal = inner
                    .as_str()
                    .parse::<bool>()
                    .map_err(ParsingError::from)
                    .map_err(Box::new)?;
                Ok(DataValue::Boolean(literal))
            }
            rule => Err(DatabaseError::from(Box::new(
                ParsingError::UnexpectedRule { rule },
            ))),
        }
    }

    /// Extracts the content of a `string_literal` `Pair`, removing the surrounding quotes.
    fn build_string_literal(&self, pair: Pair<'_, Rule>) -> String {
        let s = pair.as_str();
        s[1..s.len() - 1].to_string()
    }

    /// Builds a `CreateStmt` from a `create_stmt` `Pair`.
    ///
    /// It parses the table name, key field, and the list of field definitions.
    fn build_create_stmt(&self, pair: Pair<'_, Rule>, query: &str) -> DatabaseResult<CreateStmt> {
        let mut inner = pair.into_inner();
        let table_name = inner.next().unwrap().as_str().to_string();
        let key_field = inner.next().unwrap().as_str().to_string();
        let field_def_list_pair = inner.next().unwrap();

        let fields = field_def_list_pair
            .into_inner()
            .map(|field_def_pair| {
                let mut field_inner = field_def_pair.into_inner();
                let name = field_inner.next().unwrap().as_str().to_string();
                let data_type_pair = field_inner.next().unwrap();

                let data_type = DataType::try_from(data_type_pair.as_str())?;

                let field_def = Field { name, data_type };

                Ok(field_def)
            })
            .collect::<DatabaseResult<Vec<_>>>()?;

        Ok(CreateStmt {
            table_name,
            key_field,
            fields,
            query: query.to_string(),
        })
    }

    /// Builds an `InsertStmt` from an `insert_stmt` `Pair`.
    ///
    /// It parses the table name and the list of value assignments.
    fn build_insert_stmt(&self, pair: Pair<'_, Rule>, query: &str) -> DatabaseResult<InsertStmt> {
        let mut inner = pair.into_inner();
        let assignment_list = inner.next().unwrap();
        let table_name = inner.next().unwrap().as_str().to_string();

        let assignments = assignment_list
            .into_inner()
            .map(|assignment_pair| {
                let mut assignment_inner = assignment_pair.into_inner();
                let field_name = assignment_inner.next().unwrap().as_str().to_string();
                let value = assignment_inner.next().unwrap();

                Ok(Assignment {
                    field_name,
                    value: self.build_value(value)?,
                })
            })
            .collect::<DatabaseResult<Vec<_>>>()?;

        Ok(InsertStmt {
            table_name,
            assignments,
            query: query.to_string(),
        })
    }

    /// Builds a `DeleteStmt` from a `delete_stmt` `Pair`.
    ///
    /// It parses the target table name and the primary key value.
    fn build_delete_stmt(&self, pair: Pair<'_, Rule>, query: &str) -> DatabaseResult<DeleteStmt> {
        let mut inner = pair.into_inner();
        let key_value = inner.next().unwrap();
        let table_name = inner.next().unwrap().as_str().to_string();

        Ok(DeleteStmt {
            table_name,
            key_value: self.build_value(key_value)?,
            query: query.to_string(),
        })
    }

    /// Builds a `SelectStmt` from a `select_stmt` `Pair`.
    ///
    /// It parses the field list, the `FROM` clause, and delegates to
    /// `build_optional_clauses` for `WHERE`, `ORDER BY`, and `LIMIT`.
    fn build_select_stmt(&self, pair: Pair<'_, Rule>, query: &str) -> DatabaseResult<SelectStmt> {
        let mut inner = pair.into_inner();

        let field_list_pair = inner.next().unwrap();
        let from_clause_pair = inner.next().unwrap();

        let fields = field_list_pair
            .into_inner()
            .map(|id_pair| id_pair.as_str().to_string())
            .collect();

        let table_name = from_clause_pair
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .to_string();

        let optional_clauses = self.build_optional_clauses(inner)?;

        Ok(SelectStmt {
            fields,
            table_name,
            optional_clauses,
            query: query.to_string(),
        })
    }

    /// Parses the optional clauses (`WHERE`, `ORDER BY`, `LIMIT`) for a `SELECT` statement.
    ///
    /// It iterates over the remaining pairs from the `select_stmt` and populates
    /// an `OptionalClauses` struct.
    fn build_optional_clauses(
        &self,
        pairs: pest::iterators::Pairs<'_, Rule>,
    ) -> DatabaseResult<OptionalClauses> {
        let mut clauses = OptionalClauses::default();

        for optional_pair in pairs {
            match optional_pair.as_rule() {
                Rule::where_clause => {
                    let condition_pair = optional_pair.into_inner().next().unwrap();
                    clauses.where_clause = Some(self.build_condition(condition_pair)?);
                }
                Rule::orderby_clause => {
                    let mut order_by_inner = optional_pair.into_inner();
                    let column = order_by_inner.next().unwrap().as_str().to_string();
                    let direction_pair = order_by_inner.next().unwrap();
                    let direction = OrderDirection::try_from(direction_pair.as_str())?;
                    clauses.order_by = Some(OrderBy { column, direction });
                }
                Rule::limit_clause => {
                    let int_pair = optional_pair.into_inner().next().unwrap();
                    clauses.limit = Some(
                        int_pair
                            .as_str()
                            .parse::<i64>()
                            .map_err(ParsingError::from)
                            .map_err(Box::new)?,
                    );
                }
                rule => {
                    return Err(DatabaseError::from(Box::new(
                        ParsingError::UnexpectedRule { rule },
                    )));
                }
            }
        }

        Ok(clauses)
    }

    /// Builds the base case for a `WHERE` condition.
    ///
    /// This handles either a parenthesized `(condition)` or a simple
    /// `identifier op value` comparison.
    fn build_primary_condition(&self, pair: Pair<'_, Rule>) -> DatabaseResult<Condition> {
        let mut inner = pair.into_inner();
        let first_child = inner.next().unwrap();

        match first_child.as_rule() {
            Rule::condition => self.build_condition(first_child),
            Rule::identifier => {
                let field = first_child.as_str().to_string();
                let operator = inner.next().unwrap();
                let value = inner.next().unwrap();

                Ok(Condition::Comparison(Comparison {
                    field,
                    op: Operator::try_from(operator.as_str())?,
                    value: self.build_value(value)?,
                }))
            }
            rule => Err(DatabaseError::from(Box::new(
                ParsingError::UnexpectedRule { rule },
            ))),
        }
    }

    /// Builds a `Condition` tree for `AND` expressions.
    ///
    /// It correctly handles precedence by chaining multiple `AND`s
    /// from left to right.
    fn build_and_condition(&self, pair: Pair<'_, Rule>) -> DatabaseResult<Condition> {
        let mut inner = pair.into_inner();
        let first_child = inner.next().unwrap();
        let mut left = self.build_primary_condition(first_child)?;

        for right_pair in inner {
            let right = self.build_primary_condition(right_pair)?;
            left = Condition::And {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Builds a `Condition` tree for `OR` expressions.
    ///
    /// This is the top-level condition builder, which chains `and_condition`
    /// pairs with `OR` operators, correctly managing precedence (AND > OR).
    fn build_condition(&self, pair: Pair<'_, Rule>) -> DatabaseResult<Condition> {
        let mut inner = pair.into_inner();
        let first_child = inner.next().unwrap();
        let mut left = self.build_and_condition(first_child)?;

        for right_pair in inner {
            let right = self.build_and_condition(right_pair)?;
            left = Condition::Or {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            Assignment, Command, Comparison, Condition, CreateStmt, DeleteStmt, DumpToStmt, Field,
            InsertStmt, LoadFromStmt, Operator, OptionalClauses, OrderBy, OrderDirection,
            ReadFromStmt, SaveAsStmt, SelectStmt,
        },
        core::{DataType, DataValue},
    };

    /// Helper function to reduce boilerplate in tests.
    /// Panics if parsing fails, which is expected for valid query tests.
    fn parse_helper(query: &str) -> Command {
        let parser = QueryParser;
        parser.parse_query(query).unwrap_or_else(|e| {
            panic!(
                "Parser failed when it should have succeeded. Query: '{}', Error: {:?}",
                query, e
            )
        })
    }

    /// Helper function to test queries that are expected to fail parsing.
    fn parse_helper_fails(query: &str) {
        let parser = QueryParser;
        assert!(
            parser.parse_query(query).is_err(),
            "Parser succeeded for invalid query: '{}'",
            query
        );
    }

    #[test]
    fn test_parse_create() {
        let query = "CREATE users KEY id FIELDS id: INT, name: STRING;";
        let expected = Command::Create(CreateStmt {
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
            query: query.to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_insert() {
        let query = "INSERT id = 123, name = \"Alice\", active = true INTO users;";
        let expected = Command::Insert(InsertStmt {
            table_name: "users".to_string(),
            assignments: vec![
                Assignment {
                    field_name: "id".to_string(),
                    value: DataValue::Int(123),
                },
                Assignment {
                    field_name: "name".to_string(),
                    value: DataValue::String("Alice".to_string()),
                },
                Assignment {
                    field_name: "active".to_string(),
                    value: DataValue::Boolean(true),
                },
            ],
            query: query.to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_delete() {
        let query = "DELETE \"user-key-1\" FROM users;";
        let expected = Command::Delete(DeleteStmt {
            table_name: "users".to_string(),
            key_value: DataValue::String("user-key-1".to_string()),
            query: query.to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_select_all_clauses() {
        let query = "SELECT id, name FROM users WHERE (id > 10 OR name = \"Bob\") AND active = true ORDER_BY name DESC LIMIT 5;";

        let expected_condition = Condition::And {
            left: Box::new(Condition::Or {
                left: Box::new(Condition::Comparison(Comparison {
                    field: "id".to_string(),
                    op: Operator::Gt,
                    value: DataValue::Int(10),
                })),
                right: Box::new(Condition::Comparison(Comparison {
                    field: "name".to_string(),
                    op: Operator::Eq,
                    value: DataValue::String("Bob".to_string()),
                })),
            }),
            right: Box::new(Condition::Comparison(Comparison {
                field: "active".to_string(),
                op: Operator::Eq,
                value: DataValue::Boolean(true),
            })),
        };

        let expected = Command::Select(SelectStmt {
            fields: vec!["id".to_string(), "name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses {
                where_clause: Some(expected_condition),
                order_by: Some(OrderBy {
                    column: "name".to_string(),
                    direction: OrderDirection::Desc,
                }),
                limit: Some(5),
            },
            query: query.to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_select_simple() {
        let query = "SELECT name FROM users;";
        let expected = Command::Select(SelectStmt {
            fields: vec!["name".to_string()],
            table_name: "users".to_string(),
            optional_clauses: OptionalClauses::default(),
            query: query.to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_dump_to() {
        let query = "DUMP_TO \"data/backup.bin\";";
        let expected = Command::DumpTo(DumpToStmt {
            path: "data/backup.bin".to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_load_from() {
        let query = "LOAD_FROM \"data/backup.bin\";";
        let expected = Command::LoadFrom(LoadFromStmt {
            path: "data/backup.bin".to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_save_as() {
        let query = "SAVE_AS \"history.sql\";";
        let expected = Command::SaveAs(SaveAsStmt {
            path: "history.sql".to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_parse_read_from() {
        let query = "READ_FROM \"script.sql\";";
        let expected = Command::ReadFrom(ReadFromStmt {
            path: "script.sql".to_string(),
        });
        assert_eq!(parse_helper(query), expected);
    }

    #[test]
    fn test_invalid_query_fails() {
        parse_helper_fails("SELECT FROM users;");
        parse_helper_fails("CREATE users;");
        parse_helper_fails("INSERT users name = 1;");
        parse_helper_fails("SELECT id FROM users WHERE age =;");
        parse_helper_fails("SELECT name FROM users WHERE age > 10 AND;");
    }
}
