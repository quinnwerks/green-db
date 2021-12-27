use crate::db_error::{DbError, DbErrorType};
use crate::db_interfaces::{DbStorage, DbStorageEntry};
use std::{collections::{HashMap, hash_map::RandomState}, error::Error};
use std::marker::PhantomData;

pub struct DbSchema<T: DbStorage<B>, B: DbStorageEntry> {
    tables: HashMap<String, T>,
    entry_type: PhantomData<B>
}

impl <T: DbStorage<B>, B:DbStorageEntry> DbSchema<T,B> {
    fn new() -> DbSchema<T,B> {
        DbSchema::<T,B> {
            tables: HashMap::<String, T>::new(),
            entry_type: PhantomData
            }
    }

    fn get_entry_from_table(&self, table_name: String, entry_id: u64) -> Option<&B> {
        match self.tables.get(&table_name) {
            Some(table) => table.find_entry_by_id(entry_id),
            None => None,
        }
    }

    fn insert_entry_into_table(&mut self, table_name: String, entry: B) -> Result<(), DbError> {
        match self.tables.get(&table_name) {
            Some(table) => {
                Ok(())
            },
            None => Err(DbError::new(DbErrorType::TableNotFound)),
        }
    }
}
#[cfg(test)]
mod unit_tests {
    use crate::db_file::DbFileEntry;
    use super::*;
    
    struct MockedDbStorage<B: DbStorageEntry>  {
        expect_error: bool,
        expect_none: bool,
        entry_list: Vec<B>,
    }

    impl <B: DbStorageEntry> DbStorage<B> for MockedDbStorage<B> {
        fn find_entry_by_id(&self, id: u64) -> Option<&B> {
            if self.expect_none {
                return None
            } else {
                return self.entry_list.iter().find(|&entry| entry.get_entry_id() == id)
            }
        }   

        fn insert_entry(&mut self, entry: B) -> Result<(), DbError> {
            if self.expect_none {
                Err(DbError::new(DbErrorType::Misc))
            } else if self.expect_error {
                Err(DbError::new(DbErrorType::Misc))
            } else {
                self.entry_list.push(entry);
                Ok(())
            }
        }
    }

    

    #[test]
    fn test_get_entry_from_table__no_such_table() {
        let schema = DbSchema::<MockedDbStorage<DbFileEntry>, DbFileEntry>::new();
        //DbSchema::<MockedDbStorage<DbFileEntry>, DbFileEntry>::new();
        schema.get_entry_from_table(String::from("testy"), 2);
    }

}


/*
 #[cfg(test)]
 mod unit_tests {
     use super::*;
     use crate::db_file::DbFileEntry;
     struct MockedDbFile {
         expect_error: bool,
         entry_list: Vec<Box<dyn DbStorageEntry>>,
     
     
     #[test]
     fn test_find_entry_by_id() {
         todo!("test me");
     
     #[test]
     fn test_find_entry_by_id_non_existant_entry() {
         todo!("test me");
     }
     #[test]
     fn test_find_entry_by_id_non_existant_table() {
         todo!("test me");
     
     #[test]
     fn test_insert_entry() {
         todo!("test me");
     
     #[test]
     fn test_insert_entry_non_existant_table() {
         todo!("test me");
     }
     #[test]
     fn describe_table() {
         todo!("test me");
     
     #[test]
     fn describe_table_non_existant_table() {
         todo!("test me");
     }
 */