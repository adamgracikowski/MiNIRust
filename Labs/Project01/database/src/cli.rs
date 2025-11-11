use clap::{Arg, Command, value_parser};

use crate::core::DatabaseType;

/// A utility struct responsible for building and parsing command-line
/// interface (CLI) arguments using [`clap`].
#[derive(Debug)]
pub struct Cli;

impl Default for Cli {
    /// Creates a new, empty `Cli` instance.
    fn default() -> Self {
        Self
    }
}

impl Cli {
    /// Builds the CLI, parses the command-line arguments, and returns the selected `DatabaseType`.
    pub fn get_type(&self) -> DatabaseType {
        let cli = self.build_cli();
        let matches = cli.get_matches();
        *matches.get_one::<DatabaseType>(Self::ARG_TYPE).unwrap()
    }

    const ARG_TYPE: &'static str = "type";

    /// Defines the application's command-line interface.
    ///
    /// It specifies the app's metadata (name, version) and defines
    /// the `--type` argument, which determines the database's key type.
    fn build_cli(&self) -> Command {
        Command::new("database")
            .about("A lightweight, simple database implementation written in Rust.")
            .version("1.0.0")
            .arg(
                Arg::new("type")
                    .short('t')
                    .long("type")
                    .value_name("TYPE")
                    .value_parser(value_parser!(DatabaseType))
                    .default_value("int")
                    .help("Specifies the database type"),
            )
    }
}
