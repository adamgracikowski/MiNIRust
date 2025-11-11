//! The `parsing` module.
//!
//! This module is responsible for all query parsing. It contains the
//! `pest` parser logic (driven by the grammar) and the AST (Abstract Syntax Tree)
//! builder which converts the `pest` pairs into structured `ast::Command` objects.

mod parsing_error;
mod query_parser;

pub use parsing_error::ParsingError;
pub use query_parser::{QueryParser, Rule as QueryRule};
