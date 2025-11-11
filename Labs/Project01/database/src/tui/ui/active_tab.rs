/// Represents the currently active tab in the TUI.
#[derive(Debug, Clone, Default)]
pub enum ActiveTab {
    /// The "Queries" tab, where the user inputs queries (default).
    #[default]
    Queries,
    /// The "Database State" tab, which displays the current table schemas and data.
    DatabaseState,
}

impl ActiveTab {
    /// Returns the next tab in the cycle.
    ///
    /// This allows for simple toggling, e.g., `Queries` -> `DatabaseState` -> `Queries`.
    pub fn next(&self) -> Self {
        match self {
            Self::Queries => Self::DatabaseState,
            Self::DatabaseState => Self::Queries,
        }
    }

    /// Checks if the "Queries" tab is the currently active one.
    ///
    /// This is used by the event handler to determine if key presses
    /// should be directed to the text input buffer.
    pub fn is_queries(&self) -> bool {
        matches!(self, Self::Queries)
    }
}

/// Converts the `ActiveTab` enum into a `usize` index.
///
/// This is required by the `ratatui::widgets::Tabs` widget to know
/// which tab to select and highlight.
impl From<ActiveTab> for usize {
    fn from(value: ActiveTab) -> Self {
        match value {
            ActiveTab::Queries => 0,
            ActiveTab::DatabaseState => 1,
        }
    }
}
