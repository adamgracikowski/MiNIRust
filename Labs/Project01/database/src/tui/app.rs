use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    QueryParser,
    core::{Database, DatabaseKey},
    execution::{ExecutionResult, build_execute_command},
    tui::ui::ActiveTab,
};

pub struct App<K: DatabaseKey> {
    parser: QueryParser,

    pub database: Database<K>,
    pub input: String,
    pub cursor_position: usize,
    pub active_tab: ActiveTab,
    pub last_result: Option<Result<ExecutionResult, String>>,
    pub should_quit: bool,
}

impl<K: DatabaseKey> Default for App<K> {
    fn default() -> Self {
        Self {
            database: Database::<K>::default(),
            parser: QueryParser,
            input: String::new(),
            cursor_position: 0,
            active_tab: ActiveTab::default(),
            last_result: None,
            should_quit: false,
        }
    }
}

impl<K: DatabaseKey> App<K> {
    pub fn execute_current_query(&mut self) {
        let query_to_parse = self.input.trim();
        if query_to_parse.is_empty() {
            return;
        }

        let result = self
            .parser
            .parse_query(query_to_parse)
            .map_err(|e| format!("Parsing error:\n{}", miette::Report::new(e)))
            .and_then(|ast| {
                build_execute_command(&mut self.database, ast)
                    .map_err(|e| format!("Validation error\n{}", miette::Report::new(e)))
            })
            .and_then(|mut executable| {
                executable
                    .execute()
                    .map_err(|e| format!("Execution error:\n{}", miette::Report::new(e)))
            });

        self.last_result = Some(result);
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab => self.active_tab = self.active_tab.next(),
            KeyCode::Char(c) if self.active_tab.is_queries() => self.on_key(c),
            KeyCode::Backspace if self.active_tab.is_queries() => self.on_backspace(),
            KeyCode::Left if self.active_tab.is_queries() => self.on_left(),
            KeyCode::Right if self.active_tab.is_queries() => self.on_right(),
            KeyCode::Enter if self.active_tab.is_queries() => {
                if self.input.trim().ends_with(';') {
                    self.execute_current_query();
                } else {
                    self.on_key('\n');
                }
            }
            _ => {}
        }
    }

    pub fn on_key(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn on_backspace(&mut self) {
        if self.cursor_position > 0 {
            let prev_char_boundary = self.input[..self.cursor_position]
                .char_indices()
                .last()
                .map_or(0, |(i, _)| i);
            self.input.remove(prev_char_boundary);
            self.cursor_position = prev_char_boundary;
        }
    }

    pub fn on_left(&mut self) {
        if self.cursor_position > 0 {
            let prev_char_boundary = self.input[..self.cursor_position]
                .char_indices()
                .last()
                .map_or(0, |(i, _)| i);
            self.cursor_position = prev_char_boundary;
        }
    }

    pub fn on_right(&mut self) {
        let next_char_boundary = self.input[self.cursor_position..]
            .char_indices()
            .nth(1)
            .map_or(self.input.len(), |(i, _)| self.cursor_position + i);

        self.cursor_position = next_char_boundary;
    }
}
