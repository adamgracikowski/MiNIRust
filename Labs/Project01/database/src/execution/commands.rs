mod create;
mod delete;
mod dump_to;
mod insert;
mod load_from;
mod read_from;
mod save_as;
mod select;

use create::Create;
use delete::Delete;
use dump_to::DumpTo;
use insert::Insert;
use load_from::LoadFrom;
use read_from::ReadFrom;
use save_as::SaveAs;
use select::Select;

use crate::{
    DatabaseResult,
    ast::Command,
    core::{Database, DatabaseKey},
    execution::Execute,
};

/// The primary factory function for creating executable commands.
///
/// # Arguments
/// * `database` - A mutable reference to the active `Database` instance. The lifetime `'a`
///   is tied to this reference, ensuring that the returned trait object
///   does not outlive the database.
/// * `command` - The owned `ast::Command` produced by the parser, which will be moved
///   into the resulting `Executable` struct.
///
/// # Errors
///
/// Returns `DatabaseError::TableNotFound` if an operation (like `SELECT` or `INSERT`)
/// targets a table that does not exist.
pub fn build_execute_command<'a, K: DatabaseKey>(
    database: &'a mut Database<K>,
    command: Command,
) -> DatabaseResult<Box<dyn Execute + 'a>> {
    let executable: Box<dyn Execute> = match command {
        Command::Create(stmt) => {
            database.push_to_history(&stmt.query);
            Box::new(Create {
                ast: stmt,
                database: database,
            })
        }
        Command::Delete(stmt) => {
            database.push_to_history(&stmt.query);
            let table = database.get_table(&stmt.table_name)?;
            Box::new(Delete { table, ast: stmt })
        }
        Command::Insert(stmt) => {
            database.push_to_history(&stmt.query);
            let table = database.get_table(&stmt.table_name)?;
            Box::new(Insert { table, ast: stmt })
        }
        Command::Select(stmt) => {
            database.push_to_history(&stmt.query);
            let table = database.get_table(&stmt.table_name)?;
            Box::new(Select { table, ast: stmt })
        }
        Command::DumpTo(stmt) => Box::new(DumpTo {
            ast: stmt,
            database,
        }),
        Command::LoadFrom(stmt) => Box::new(LoadFrom {
            ast: stmt,
            database,
        }),
        Command::SaveAs(stmt) => Box::new(SaveAs {
            ast: stmt,
            database,
        }),
        Command::ReadFrom(stmt) => Box::new(ReadFrom {
            ast: stmt,
            database,
        }),
    };

    Ok(executable)
}
