use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct DbError {
    error_type: DbErrorType,
}

#[derive(Debug)]
pub enum DbErrorType {
    TableNotFound,
    Misc,
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Error:\"{:?}\"", self.error_type)
    }
}
impl Error for DbError {}
impl DbError {
    pub fn new(errorType: DbErrorType) -> DbError {
        DbError {
            error_type: errorType,
        }
    }
}
