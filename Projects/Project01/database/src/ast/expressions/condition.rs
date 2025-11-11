use super::Comparison;

/// Represents a recursive condition for a `WHERE` clause.
///
/// This enum forms a tree structure that allows for combining simple comparisons
/// using logical `AND` and `OR` operators, enabling complex filtering logic
/// like `(age > 21 OR name = "John") AND id < 100`.
#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    /// Represents a logical `OR` operation between two sub-conditions.
    ///
    /// The condition is true if *either* the `left` or the `right` sub-condition is true.
    Or {
        /// The left-hand side of the `OR` operation.
        left: Box<Condition>,
        /// The right-hand side of the `OR` operation.
        right: Box<Condition>,
    },
    /// Represents a logical `AND` operation between two sub-conditions.
    ///
    /// The condition is true only if *both* the `left` and the `right` sub-conditions are true.
    And {
        /// The left-hand side of the `AND` operation.
        left: Box<Condition>,
        /// The right-hand side of the `AND` operation.
        right: Box<Condition>,
    },
    /// A leaf node in the condition tree, representing a single, atomic comparison.
    ///
    /// e.g., `age > 21`
    Comparison(Comparison),
}
