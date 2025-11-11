use std::{collections::HashMap, fmt};

use bincode::{Decode, Encode};

use crate::core::DataValue;

/// Represents a single record or row within a table.
///
/// It stores the actual data as a map where the key is the column (field) name
/// and the value is the corresponding `DataValue`.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Record {
    /// A map holding the data for this record, associating column names with their values.
    pub fields: HashMap<String, DataValue>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields: Vec<String> = self
            .fields
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect();

        fields.sort();

        write!(f, "{{ {} }}", fields.join(", "))
    }
}
