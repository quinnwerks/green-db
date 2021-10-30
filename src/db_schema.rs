use crate::db_error::{DbError, DbErrorType};
use crate::db_interfaces::{DbStorage, DbStorageEntry};
use std::collections::HashMap;

pub struct DbSchema {
    tables: HashMap<String, Box<dyn DbStorage>>,
}

impl DbSchema {
    fn find_entry_by_id_for_table(
        &self,
        entry_id: u64,
        table_name: String,
    ) -> Option<Box<&dyn DbStorageEntry>> {
        match self.tables.get(&table_name) {
            Some(table) => match table.find_entry_by_id(entry_id) {
                Some(entry) => Some(Box::new(entry)),
                None => None,
            },
            None => None,
        }
    }

    fn insert_entry_into_table(
        &self,
        entry: Box<dyn DbStorageEntry>,
        table_name: String,
    ) -> Result<(), DbError> {
        match self.tables.get(&table_name) {
            Some(table) => table.insert_entry(entry),
            None => Err(DbError::new(DbErrorType::TableNotFound)),
        }
    }

    fn describe_table(&self, table_name: String) {
        todo!("implement me")
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_find_entry_by_id() {
        todo!("test me");
    }

    #[test]
    fn test_find_entry_by_id_non_existant_entry() {
        todo!("test me");
    }
    #[test]
    fn test_find_entry_by_id_non_existant_table() {
        todo!("test me");
    }

    #[test]
    fn test_insert_entry() {
        todo!("test me");
    }

    #[test]
    fn test_insert_entry_non_existant_table() {
        todo!("test me");
    }
    #[test]
    fn describe_table() {
        todo!("test me");
    }

    #[test]
    fn describe_table_non_existant_table() {
        todo!("test me");
    }
}
