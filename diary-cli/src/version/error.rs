use std::{fmt, error::Error};

#[derive(Debug)]
pub enum VersionError {
    InvalidSeparator(String),
    InvalidVersion,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VersionError: {:?}", self)
    }
}

impl Error for VersionError {}