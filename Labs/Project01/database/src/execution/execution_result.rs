use std::fmt;

use crate::core::Record;

/// Represents the successful result of executing any database command.
///
/// This enum standardizes the different types of successful outcomes
/// from various commands.
#[derive(Debug)]
pub enum ExecutionResult {
    /// Indicates successful execution for commands that do not return data
    /// or modify rows (e.g., `CREATE`, `DUMP_TO`, `SAVE_AS`).
    Success,
    /// Indicates the number of rows modified by a command
    /// (e.g., `INSERT`, `DELETE`).
    RowsAffected(usize),
    /// Contains a vector of `Record`s returned by a query (e.g., `SELECT`).
    Data(Vec<Record>),
    /// Contains a list of informational messages generated during execution
    /// (e.g., from a `READ_FROM` script).
    Messages(Vec<String>),
}

impl fmt::Display for ExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionResult::Success => write!(f, "Success"),
            ExecutionResult::RowsAffected(n) => write!(f, "{n} row(s) affected"),
            ExecutionResult::Data(records) => {
                if records.is_empty() {
                    writeln!(f, "No data")
                } else {
                    writeln!(f, "Returned {} record(s):", records.len())?;
                    for (i, record) in records.iter().enumerate() {
                        writeln!(f, "{:>3}: {record}", i + 1)?;
                    }
                    Ok(())
                }
            }
            ExecutionResult::Messages(msgs) => {
                if msgs.is_empty() {
                    writeln!(f, "No messages")
                } else {
                    writeln!(f, "Messages:")?;
                    for msg in msgs {
                        writeln!(f, "- {msg}")?;
                    }
                    Ok(())
                }
            }
        }
    }
}
