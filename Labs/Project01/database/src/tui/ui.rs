mod active_tab;
mod database_state_tab;
mod queries_tab;
mod widgets;

use std::usize;

pub use active_tab::ActiveTab;
pub use database_state_tab::create_database_state_tab;
pub use queries_tab::create_queries_tab;
pub use widgets::{create_records_table, create_schema_table};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Tabs},
};

use crate::{core::DatabaseKey, tui::App};

pub fn ui<K: DatabaseKey>(f: &mut Frame, app: &mut App<K>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    let titles = vec![" [1] Queries ", " [2] Database State "];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Database "))
        .select(usize::from(app.active_tab.clone()))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    match app.active_tab {
        ActiveTab::Queries => create_queries_tab(f, app, chunks[1]),
        ActiveTab::DatabaseState => create_database_state_tab(f, app, chunks[1]),
    };
}
