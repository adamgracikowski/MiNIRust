use std::collections::{BTreeMap, HashMap};

use bincode::{Decode, Encode};

use crate::{
    DatabaseResult,
    core::{DataType, DatabaseError},
};

use super::{key::DatabaseKey, record::Record};

/// Represents a single table within the database.
///
/// A table is generic over its primary key type `K` and stores both its
/// schema (column definitions) and its data (rows).
#[derive(Debug, Clone, Encode, Decode)]
pub struct Table<K: DatabaseKey> {
    /// The name of the table.
    pub name: String,
    /// The name of the field within the schema that serves as the primary key.
    pub key_field: String,
    /// The table's schema, mapping column names to their respective `DataType`.
    pub schema: HashMap<String, DataType>,
    /// The actual data stored as rows.
    ///
    /// A `BTreeMap` is used to store rows, mapping the unique primary key (of type `K`)
    /// to the full `Record` object. This ensures keys are sorted and unique.
    pub rows: BTreeMap<K, Record>,
}

impl<K: DatabaseKey> Table<K> {
    /// Creates and validates a new, empty table.
    ///
    /// This constructor performs critical validation to ensure the table's
    /// integrity before creation:
    /// 1. It verifies that the specified `key_field` exists within the `schema`.
    /// 2. It verifies that the `DataType` of the `key_field` in the schema
    ///    matches the `DataType` associated with the generic key type `K`.
    ///    (e.g., `K=i64` must match `DataType::Int`).
    ///
    /// # Arguments
    /// * `name` - The name for the new table.
    /// * `key_field` - The name of the column that will act as the primary key.
    /// * `schema` - A `HashMap` defining all columns and their types. This map is consumed.
    ///
    /// # Errors
    /// Returns `DatabaseError::FieldNotFound` if the `key_field` is not in the `schema`.
    /// Returns `DatabaseError::TypeMismatch` if the `key_field`'s type in the schema
    /// does not match the generic type `K`.
    pub fn new(
        name: &str,
        key_field: &str,
        schema: HashMap<String, DataType>,
    ) -> DatabaseResult<Self> {
        let key_schema_type =
            schema
                .get(key_field)
                .ok_or_else(|| DatabaseError::FieldNotFound {
                    table: name.to_string(),
                    field: key_field.to_string(),
                })?;

        let generic_key_type = K::key_type();

        if *key_schema_type != generic_key_type {
            return Err(DatabaseError::TypeMismatch {
                table: name.to_string(),
                field: key_field.to_string(),
                expected: generic_key_type,
                found: *key_schema_type,
            });
        }

        Ok(Self {
            name: name.to_string(),
            key_field: key_field.to_string(),
            schema,
            rows: BTreeMap::new(),
        })
    }
}
