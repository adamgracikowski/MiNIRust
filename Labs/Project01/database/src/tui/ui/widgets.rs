use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, Table},
};
use unicode_width::UnicodeWidthStr;

use crate::core::{DataType, Record};

pub fn create_schema_table(
    f: &mut Frame,
    block: Block,
    schema: &HashMap<String, DataType>,
    area: Rect,
) {
    let headers = ["Column", "Type"];
    let header_cells: Vec<Cell> = headers
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)))
        .collect();
    let header_row = Row::new(header_cells).bottom_margin(1);

    let mut sorted_schema: Vec<(&String, &DataType)> = schema.iter().collect();
    sorted_schema.sort_by(|a, b| a.0.cmp(b.0));

    let rows: Vec<Row> = sorted_schema
        .iter()
        .map(|(name, dtype)| {
            let cells = vec![
                Cell::from(name.as_str()),
                Cell::from(format!("{:?}", dtype)).style(Style::default().fg(Color::Yellow)),
            ];
            Row::new(cells)
        })
        .collect();

    let widths_constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];

    let t = Table::new(rows, widths_constraints)
        .header(header_row)
        .block(block);

    f.render_widget(t, area);
}

pub fn create_records_table(f: &mut Frame, block: Block, records: &[Record], area: Rect) {
    if records.is_empty() {
        f.render_widget(Paragraph::new("No data.").block(block), area);
        return;
    }

    let first_record = &records[0];
    let mut headers: Vec<String> = first_record.fields.keys().cloned().collect();
    headers.sort();

    let header_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .fg(Color::Yellow);
    let alt_row_style = Style::default().bg(Color::Rgb(30, 30, 30));

    let rows: Vec<Row> = records
        .iter()
        .enumerate()
        .map(|(i, record)| {
            let cells: Vec<Cell> = headers
                .iter()
                .map(|header| match record.fields.get(header) {
                    Some(val) => Cell::from(format!("{val}")),
                    None => Cell::from("NULL").style(Style::default().fg(Color::DarkGray)),
                })
                .collect();

            if i % 2 == 1 {
                Row::new(cells).style(alt_row_style)
            } else {
                Row::new(cells)
            }
        })
        .collect();

    let widths_constraints = calculate_column_widths(&headers, records);

    let header_cells: Vec<Cell> = headers
        .iter()
        .map(|h| Cell::from(h.as_str()).style(header_style))
        .collect();
    let header_row = Row::new(header_cells).bottom_margin(1);

    let t = Table::new(rows, widths_constraints)
        .header(header_row)
        .block(block)
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    f.render_widget(t, area);
}

fn calculate_column_widths(headers: &[String], records: &[Record]) -> Vec<Constraint> {
    let mut widths: Vec<usize> = headers.iter().map(|h| h.width()).collect();

    for record in records {
        for (i, header) in headers.iter().enumerate() {
            let value_width = record
                .fields
                .get(header)
                .map_or(4, |val| val.to_string().width());

            if value_width > widths[i] {
                widths[i] = value_width;
            }
        }
    }

    widths
        .into_iter()
        .map(|w| Constraint::Length(w as u16 + 2))
        .collect()
}
