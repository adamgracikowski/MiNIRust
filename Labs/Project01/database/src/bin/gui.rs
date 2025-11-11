use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use miette::{IntoDiagnostic, Result};
use ratatui::prelude::*;

use database::{
    Cli,
    core::{DatabaseKey, DatabaseType},
    tui::{App, ui},
};

fn main() -> Result<()> {
    let cli = Cli;
    let database_type = cli.get_type();

    match database_type {
        DatabaseType::Int => run_tui_loop::<i64>()?,
        DatabaseType::String => run_tui_loop::<String>()?,
    }

    Ok(())
}

fn run_tui_loop<K: DatabaseKey>() -> Result<()> {
    enable_raw_mode().into_diagnostic()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).into_diagnostic()?;

    let mut app = App::<K>::default();

    loop {
        terminal.draw(|f| ui(f, &mut app)).into_diagnostic()?;

        if event::poll(Duration::from_millis(250)).into_diagnostic()?
            && let Event::Key(key) = event::read().into_diagnostic()?
            && key.kind == KeyEventKind::Press
        {
            app.handle_key_event(key);
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode().into_diagnostic()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).into_diagnostic()?;
    terminal.show_cursor().into_diagnostic()?;

    Ok(())
}
