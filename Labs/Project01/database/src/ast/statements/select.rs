use crate::ast::{clauses::OrderBy, expressions::Condition};

/// Represents a `SELECT` SQL statement.
///
/// This struct holds all parsed information for a query, including the columns
/// to return (`fields`), the table to query (`table_name`), and any
/// optional clauses like `WHERE`, `ORDER BY`, or `LIMIT`.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    /// A vector of column names to be selected.
    /// e.g., `["id", "name"]`
    pub fields: Vec<String>,
    /// The name of the table to query.
    /// e.g., `"people"`
    pub table_name: String,
    /// A struct containing all optional clauses (`WHERE`, `ORDER BY`, `LIMIT`).
    pub optional_clauses: OptionalClauses,
    /// The raw, original query string that was parsed to create this statement.
    pub query: String,
}

/// A container for all optional clauses that can accompany a `SELECT` statement.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OptionalClauses {
    /// The optional `WHERE` clause, represented as a tree of conditions.
    pub where_clause: Option<Condition>,
    /// The optional `ORDER BY` clause, specifying sort column and direction.
    pub order_by: Option<OrderBy>,
    /// The optional `LIMIT` clause, specifying the maximum number of rows to return.
    pub limit: Option<i64>,
}
