//! The main library crate for the database engine.
//!
//! This crate encapsulates all the core logic for the database,
//! including parsing, execution, and data modeling. It exposes a public
//! API for frontends (like the CLI or TUI) to consume.

mod ast;
mod cli;
mod parsing;

pub mod core;
pub mod execution;
pub mod tui;

pub use cli::Cli;
pub use parsing::{QueryParser, QueryRule};

pub use core::DatabaseResult;
