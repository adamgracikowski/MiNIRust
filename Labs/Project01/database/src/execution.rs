//! The `execution` module.
//!
//! This module contains all logic for the **command execution phase**.
//! It defines the `Execute` trait, holds the concrete implementation for each
//! command (e.g., `Create`, `Select`), and provides the `build_execute_command`
//! factory function to bridge the AST and the executor.

mod commands;
mod execute;
mod execution_result;

pub use commands::build_execute_command;
pub use execute::Execute;
pub use execution_result::ExecutionResult;
