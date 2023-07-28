use std::fmt;

impl std::error::Error for OFileError {}
#[derive(Debug)]
pub enum OFileError {
    CannotWriteToFile(String),
    CannotReadFile(String),
}

impl fmt::Display for OFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use OFileError::*;
        match self {
            CannotWriteToFile(p) => write!(f, "Cannot write to file '{p}' while in read-only mode"),
            CannotReadFile(p) => write!(f, "Cannot read file '{p}' while in write-only mode"),
        }
    }
}