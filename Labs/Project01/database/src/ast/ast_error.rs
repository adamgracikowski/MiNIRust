use miette::Diagnostic;
use thiserror::Error;

use crate::{ast::expressions::Operator, core::DataType};

/// Represents errors that can occur during the AST (Abstract Syntax Tree)
/// building or validation phase.
#[derive(Error, Debug, Diagnostic)]
pub enum AstError {
    #[error("Unknown order: `{order}`")]
    UnknownOrder { order: String },

    #[error("Unknown operator: `{operator}`")]
    UnknownOperator { operator: String },

    #[error("Invalid operator '{operator:?}' for type `{dtype:?}`")]
    InvalidOperatorForType { operator: Operator, dtype: DataType },
}
