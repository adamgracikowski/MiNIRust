#[derive(Debug, Clone, Default)]
pub enum ActiveTab {
    #[default]
    Queries,
    DatabaseState,
}

impl ActiveTab {
    pub fn next(&self) -> Self {
        match self {
            Self::Queries => Self::DatabaseState,
            Self::DatabaseState => Self::Queries,
        }
    }

    pub fn is_queries(&self) -> bool {
        matches!(self, Self::Queries)
    }
}

impl From<ActiveTab> for usize {
    fn from(value: ActiveTab) -> Self {
        match value {
            ActiveTab::Queries => 0,
            ActiveTab::DatabaseState => 1,
        }
    }
}
