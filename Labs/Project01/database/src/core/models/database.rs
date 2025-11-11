use std::collections::HashMap;
use std::collections::hash_map::Entry;

use bincode::{Decode, Encode};
use clap::ValueEnum;

use crate::DatabaseResult;
use crate::core::{DatabaseError, errors::CreateError};

use super::{key::DatabaseKey, table::Table};

/// Specifies the database variant based on its primary key type.
///
/// This is used by `clap` to parse command-line arguments.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DatabaseType {
    /// Use `i64` as the primary key type.
    Int,
    /// Use `String` as the primary key type.
    String,
}

/// Represents the top-level database instance.
///
/// It is generic over the primary key type `K` and holds all tables
/// and a history of executed commands.
#[derive(Debug, Encode, Decode)]
pub struct Database<K: DatabaseKey> {
    /// A map of table names to their corresponding `Table` structs.
    pub tables: HashMap<String, Table<K>>,
    /// A log of all successfully executed query strings, used for `SAVE_AS`.
    history: Vec<String>,
}

/// Creates a new, empty `Database` instance.
///
/// Initializes an empty table map and an empty command history.
impl<K: DatabaseKey> Default for Database<K> {
    fn default() -> Self {
        Self {
            tables: Default::default(),
            history: Default::default(),
        }
    }
}

impl<K: DatabaseKey> Database<K> {
    /// Adds a new table to the database.
    ///
    /// # Arguments
    /// * `table` - The `Table` object to add.
    ///
    /// # Errors
    /// Returns `DatabaseError` (wrapping `CreateError::TableAlreadyExists`) if a
    /// table with the same name already exists.
    pub fn add_table(&mut self, table: Table<K>) -> DatabaseResult<bool> {
        let table_name = table.name.clone();
        match self.tables.entry(table_name) {
            Entry::Occupied(entry) => Err(CreateError::TableAlreadyExists {
                name: entry.key().to_string(),
            }
            .into()),
            Entry::Vacant(entry) => {
                entry.insert(table);
                Ok(true)
            }
        }
    }

    /// Retrieves a mutable reference to a table by its name.
    ///
    /// # Arguments
    /// * `table_name` - The name of the table to retrieve.
    ///
    /// # Errors
    /// Returns `DatabaseError::TableNotFound` if no table with that name exists.
    pub fn get_table(&mut self, table_name: &str) -> DatabaseResult<&mut Table<K>> {
        self.tables
            .get_mut(table_name)
            .ok_or_else(|| DatabaseError::TableNotFound {
                name: table_name.to_string(),
            })
    }

    /// Appends a raw query string to the command history.
    ///
    /// This is typically called after a command has been successfully executed.
    /// The history is used for the `SAVE_AS` command.
    pub fn push_to_history(&mut self, command: &str) {
        self.history.push(command.to_string());
    }

    /// Returns an iterator over all commands stored in the history.
    pub fn iter_history(&self) -> impl Iterator<Item = &String> {
        self.history.iter()
    }
}