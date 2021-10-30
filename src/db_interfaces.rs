use crate::db_error::DbError;

pub trait DbStorage {
    fn find_entry_by_id(&self, id: u64) -> Option<&dyn DbStorageEntry>;
    fn insert_entry(&self, entry: Box<dyn DbStorageEntry>) -> Result<(), DbError>;
}

pub trait DbStorageEntry {
    fn set_entry_id(&self, id: u64);
    fn set_entry_data(&self, data: Vec<u8>);
    fn set_entry_alive(&self, alive: bool);

    fn get_entry_id(&self) -> u64;
    fn get_entry_alive(&self) -> bool;
    fn get_entry_data(&self) -> Vec<u8>;
    fn get_entry_size(&self) -> u64;
}
