mod repl;

use database::{Cli, core::DatabaseType};
use miette::Result;

use crate::repl::run_repl;

fn main() -> Result<()> {
    miette::set_panic_hook();

    let cli = Cli::default();
    let database_type = cli.get_type();

    match database_type {
        DatabaseType::Int => run_repl::<i64>()?,
        DatabaseType::String => run_repl::<String>()?,
    }

    Ok(())
}
