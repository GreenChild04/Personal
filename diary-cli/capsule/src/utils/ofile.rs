pub mod error;
pub mod ofile_mode;
pub use error::*;
pub use ofile_mode::*;

use crate::prelude::*;
use std::path::Path;
use crate::unwrap_result;
use std::fs::File;
use std::io::{Write, Read};

pub struct OFile {
    file_path: String,
    mode: OFileModeInternal,
    idx: u64,
}

impl OFile {
    pub fn new(file_path: String, mode: OFileMode) -> Result<Self, CapError> {
        let file = match mode {
            OFileMode::Write => unwrap_result!(File::create(&file_path) => |e| Err(CapError::IOError(e))),
            OFileMode::Read => {
                if !Path::new(&file_path).exists() { return Err(CapError::FileNotFound(file_path)) }
                unwrap_result!(File::open(&file_path) => |e| Err(CapError::IOError(e)))
            },
            OFileMode::Modify => {
                if !Path::new(&file_path).exists() { return Err(CapError::FileNotFound(file_path)) }
                unwrap_result!(File::open(&file_path) => |e| Err(CapError::IOError(e)))
            },
        };

        Ok(Self {
            mode: OFileModeInternal::from(mode, file, &file_path)?,
            file_path,
            idx: 0,
        })
    }

    pub fn read(&mut self) -> Result<u8, OFileError> {
        // Get access to reader to read...
        // let reader = if let OFileModeInterna
        todo!();
    }

    pub fn write(&mut self, value: u8) -> Result<(), OFileError> {
        // Get access to writer to write...
        let writer = if let OFileModeInternal::Write(w) = &self.mode { w }
        else { return Err(OFileError::CannotWriteToFile(self.file_path.clone())) };

        todo!();
    }
}