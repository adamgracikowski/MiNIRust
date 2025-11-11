//! The Abstract Syntax Tree (AST) module.
//!
//! This module defines the complete set of data structures that represent
//! the parsed form of a SQL query *before* it is validated or executed.
//!
//! The `parser` is responsible for producing these structures, and the
//! `executor` is responsible for consuming them. The primary entry point
//! is the [Command] enum.

mod ast_error;
mod clauses;
mod expressions;
mod statements;

pub use ast_error::AstError;
pub use clauses::*;
pub use expressions::*;
pub use statements::*;

/// Represents a single, complete, parsed command.
///
/// This is the root of the AST and the primary output of the `QueryParser`.
/// Each variant corresponds to a top-level SQL statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// A `CREATE TABLE` statement.
    Create(CreateStmt),
    /// An `INSERT INTO` statement.
    Insert(InsertStmt),
    /// A `DELETE FROM` statement.
    Delete(DeleteStmt),
    /// A `SELECT` statement.
    Select(SelectStmt),
    /// A `SAVE_AS` (history) statement.
    SaveAs(SaveAsStmt),
    /// A `READ_FROM` (script) statement.
    ReadFrom(ReadFromStmt),
    /// A `DUMP_TO` (binary) statement.
    DumpTo(DumpToStmt),
    /// A `LOAD_FROM` (binary) statement.
    LoadFrom(LoadFromStmt),
}
