use crate::ast::AstError;

/// Represents an `ORDER BY` clause, specifying a column and a sort direction.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBy {
    /// The name of the column to sort by.
    pub column: String,
    /// The direction of the sort (Ascending or Descending).
    pub direction: OrderDirection,
}

/// Specifies the direction of sorting for an `ORDER BY` clause.
#[derive(Debug, Clone, PartialEq)]
pub enum OrderDirection {
    /// Ascending order (e.g., A-Z, 1-10).
    Asc,
    /// Descending order (e.g., Z-A, 10-1).
    Desc,
}

/// Enables parsing an `OrderDirection` from a string slice.
///
/// This is used by the parser to convert the "ASC" or "DESC" tokens
/// into the corresponding enum variant.
impl TryFrom<&str> for OrderDirection {
    type Error = AstError;

    /// Attempts to parse a string slice into an `OrderDirection`.
    ///
    /// # Errors
    ///
    /// Returns `AstError::UnknownOrder` if the string is not
    /// precisely "ASC" or "DESC".
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let order = match value {
            "ASC" => OrderDirection::Asc,
            "DESC" => OrderDirection::Desc,
            // Catch any other string and return an error
            order => {
                return Err(AstError::UnknownOrder {
                    order: order.to_string(),
                });
            }
        };

        Ok(order)
    }
}
