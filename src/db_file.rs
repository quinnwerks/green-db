use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct DbFileHeader {
    data_size: u64,
    num_entries: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DbFileEntry {
    id: u64,
    alive: bool,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct DbFile {
    header: DbFileHeader,
    path: PathBuf,
}

impl PartialEq for DbFile {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.path == other.path
    }
}
impl Eq for DbFile {}

impl DbFile {
    pub fn new_to_disk(data_size: u64, path: &PathBuf) -> Result<DbFile, Error> {
        let db_file = DbFile::new_in_mem(data_size, 0, path);
        db_file.to_disk()?;
        Ok(db_file)
    }

    pub fn new_from_disk(path: &PathBuf) -> Result<DbFile, Error> {
        let fd = File::open(&path)?;
        let header = DbFile::read_header(fd)?;
        Ok(DbFile {
            header: header,
            path: PathBuf::from(path),
        })
    }

    pub fn append_entry(&self, fd: &File, entry: &DbFileEntry) -> Result<(), Error> {
        self.write_entry_at(&fd, SeekFrom::End(0), entry)
    }

    fn new_in_mem(data_size: u64, num_entries: u64, path: &PathBuf) -> DbFile {
        DbFile {
            header: DbFileHeader {
                data_size: data_size,
                num_entries: num_entries,
            },
            path: PathBuf::from(path),
        }
    }

    fn to_disk(&self) -> Result<(), Error> {
        let fd = File::create(&self.path)?;
        self.write_header(fd)
    }

    fn write_header(&self, mut fd: File) -> Result<(), Error> {
        fd.write_all(&self.header.data_size.to_ne_bytes())?;
        fd.write_all(&self.header.num_entries.to_ne_bytes())?;
        Ok(())
    }

    fn read_header(mut fd: File) -> Result<DbFileHeader, Error> {
        let mut data_size_raw: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
        let mut num_entries_raw: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
        fd.read_exact(&mut data_size_raw)?;
        fd.seek(SeekFrom::Start(size_of::<u64>() as u64))?;
        fd.read_exact(&mut num_entries_raw)?;
        let data_size = u64::from_ne_bytes(data_size_raw);
        let num_entries = u64::from_ne_bytes(num_entries_raw);
        Ok(DbFileHeader {
            data_size,
            num_entries,
        })
    }

    pub fn read_entry_at(&self, mut fd: &File, offset: u64) -> Result<DbFileEntry, Error> {
        let mut id_raw: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
        let mut alive_raw: [u8; size_of::<bool>()] = [0; size_of::<bool>()];
        let mut data = vec![0; self.header.data_size as usize];
        fd.seek(SeekFrom::Start(offset))?;
        fd.read_exact(&mut id_raw)?;
        fd.seek(SeekFrom::Start(offset + size_of::<u64>() as u64))?;
        fd.read_exact(&mut alive_raw)?;
        fd.seek(SeekFrom::Start(
            offset + (size_of::<u64>() + size_of::<bool>()) as u64,
        ))?;
        fd.read_exact(&mut data)?;
        println!("{:?}", alive_raw);
        Ok(DbFileEntry {
            id: u64::from_ne_bytes(id_raw),
            alive: alive_raw[0] != 0,
            data: Vec::from(data),
        })
    }

    fn write_entry_at(
        &self,
        mut fd: &File,
        location: SeekFrom,
        entry: &DbFileEntry,
    ) -> Result<(), Error> {
        fd.seek(location)?;
        fd.write(&entry.id.to_ne_bytes())?;
        fd.write(&[entry.alive as u8])?;
        fd.write(&entry.data)?;
        Ok(())
    }

    fn get_entry_size(&self) -> usize {
        size_of::<u64>() + self.header.data_size as usize
    }
}

#[cfg(test)]
mod integ_tests {
    use std::fs::remove_file;
    use std::fs::OpenOptions;

    struct IntegTest {
        path: PathBuf,
    }
    impl Drop for IntegTest {
        fn drop(&mut self) {
            match remove_file(&self.path) {
                Err(why) => panic!("{}", why),
                _ => (),
            }
        }
    }

    use super::*;
    #[test]
    fn test_create_and_read_db_file() -> Result<(), Error> {
        let setup = IntegTest {
            path: PathBuf::from("test_create_and_read_db_file.db"),
        };

        let db_file = DbFile::new_to_disk(20, &setup.path)?;
        let db_file_read = DbFile::new_from_disk(&setup.path)?;

        assert_eq!(db_file, db_file_read);
        return Ok(());
    }

    #[test]
    fn test_create_and_read_db_entry() -> Result<(), Error> {
        let setup = IntegTest {
            path: PathBuf::from("test_create_and_read_db_entry.db"),
        };
        let db_file = DbFile::new_to_disk(3, &setup.path)?;
        let new_entry = DbFileEntry {
            id: 5,
            alive: true,
            data: Vec::from(String::from("abc").as_bytes()),
        };

        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&setup.path)?;
        match db_file.append_entry(&fd, &new_entry) {
            Err(why) => panic!("{}", why),
            _ => (),
        };

        let new_entry_read = db_file.read_entry_at(&fd, size_of::<DbFileHeader>() as u64)?;
        assert_eq!(new_entry, new_entry_read);
        return Ok(());
    }
}
