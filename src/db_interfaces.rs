use crate::db_error::DbError;

pub trait DbStorage<T: DbStorageEntry> {
    fn find_entry_by_id(&self, id: u64) -> Option<&T>;
    fn insert_entry(&mut self, entry: T) -> Result<(), DbError>;
}

pub trait DbStorageEntry {
    fn set_entry_id(&mut self, id: u64);
    fn set_entry_data(&mut self, data: Vec<u8>);
    fn set_entry_alive(&mut self, alive: bool);

    fn get_entry_id(&self) -> u64;
    fn get_entry_alive(&self) -> bool;
    fn get_entry_data(&self) -> &Vec<u8>;
    fn get_entry_size(&self) -> u64;
}
