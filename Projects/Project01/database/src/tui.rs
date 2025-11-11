//! The main TUI (Text User Interface) module.
//!
//! This module acts as the root for the TUI frontend. It organizes the TUI
//! logic into state management (`app`) and rendering (`ui`).

mod app;
mod ui;

pub use app::App;
pub use ui::ui;
