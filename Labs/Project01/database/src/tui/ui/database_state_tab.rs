use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

use crate::{core::DatabaseKey, tui::App};

use super::{create_records_table, create_schema_table};

pub fn create_database_state_tab<K: DatabaseKey>(f: &mut Frame, app: &App<K>, area: Rect) {
    let table_names: Vec<&String> = app.database.tables.keys().collect();
    if table_names.is_empty() {
        f.render_widget(
            Paragraph::new("Database is empty.").block(Block::default().borders(Borders::ALL)),
            area,
        );
        return;
    }

    let constraints: Vec<Constraint> = table_names
        .iter()
        .map(|_| Constraint::Ratio(1, table_names.len() as u32))
        .collect();
    let table_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, table_name) in table_names.into_iter().enumerate() {
        let table = app.database.tables.get(table_name).unwrap();

        let table_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(table_chunks[i]);

        let schema_title = format!(" Schema: {} (Key: {}) ", table.name, table.key_field);
        let schema_block = Block::default().borders(Borders::ALL).title(schema_title);
        create_schema_table(f, schema_block, &table.schema, table_layout[0]);

        let data_title = format!(" Rows ");
        let data_block = Block::default().borders(Borders::ALL).title(data_title);
        let records: Vec<_> = table.rows.values().cloned().collect();
        create_records_table(f, data_block, &records, table_layout[1]);
    }
}
