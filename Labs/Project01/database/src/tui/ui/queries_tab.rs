use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{core::DatabaseKey, execution::ExecutionResult, tui::App};

use super::create_records_table;

/// Renders the "Queries" tab view.
pub fn create_queries_tab<K: DatabaseKey>(f: &mut Frame, app: &App<K>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)])
        .split(area);

    let input_paragraph = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Enter query (finish with ';', press Enter) "),
        );
    f.render_widget(input_paragraph, chunks[0]);

    let text_before_cursor = &app.input[..app.cursor_position];
    let lines_before: Vec<&str> = text_before_cursor.split('\n').collect();

    let cursor_y = (lines_before.len() - 1) as u16;
    let cursor_x = lines_before.last().map_or(0, |line| line.chars().count()) as u16;

    let position = Position::new(chunks[0].x + cursor_x + 1, chunks[0].y + cursor_y + 1);
    f.set_cursor_position(position);

    let output_block = Block::default().borders(Borders::ALL).title(" Result ");

    if let Some(result) = &app.last_result {
        match result {
            Ok(exec_result) => match exec_result {
                ExecutionResult::Data(records) => {
                    create_records_table(f, output_block, records, chunks[1]);
                }
                ExecutionResult::RowsAffected(count) => {
                    let text = format!("{count} row(s) affected.");
                    f.render_widget(Paragraph::new(text).block(output_block).green(), chunks[1]);
                }
                ExecutionResult::Success => {
                    f.render_widget(
                        Paragraph::new("Success.").block(output_block).green(),
                        chunks[1],
                    );
                }
                ExecutionResult::Messages(log_lines) => {
                    let text = log_lines.join("\n");
                    f.render_widget(
                        Paragraph::new(text)
                            .block(output_block.title(" Results "))
                            .wrap(Wrap { trim: true })
                            .green(),
                        chunks[1],
                    );
                }
            },
            Err(error_msg) => {
                f.render_widget(
                    Paragraph::new(error_msg.as_str())
                        .block(output_block)
                        .wrap(Wrap { trim: true })
                        .red(),
                    chunks[1],
                );
            }
        }
    } else {
        f.render_widget(
            Paragraph::new("Enter query...")
                .block(output_block)
                .dark_gray(),
            chunks[1],
        );
    }
}
