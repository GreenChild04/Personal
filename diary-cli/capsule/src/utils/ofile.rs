pub mod error;
pub mod ofile_mode;
pub use error::*;
pub use ofile_mode::*;

use std::path::Path;
use crate::unwrap_result;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub struct OFile {
    file_path: String,
    mode: OFileMode,
    current: Option<u8>,
    idx: u64,
}

impl OFile {
    pub fn new<'a>(file_path: String) -> Result<Self, OFileError<'a>> {
        if Path::new(&file_path).exists() {
            Self::new_read(file_path)
        } else { Self::new_write(file_path) }
    }

    fn new_write<'a>(file_path: String) -> Result<Self, OFileError<'a>> {
        let file = unwrap_result!(File::create(&file_path) => |e| Err(OFileError::IOError(e)));
        Ok(Self {
            file_path,
            mode: OFileMode::new_write(file),
            idx: 0,
            current: None,
        })
    }

    fn new_read<'a>(file_path: String) -> Result<Self, OFileError<'a>> {
        let file = unwrap_result!(File::open(&file_path) => |e| Err(OFileError::IOError(e)));
        Ok(Self {
            file_path,
            mode: OFileMode::new_read(file),
            idx: 0,
            current: None,
        })
    }

    pub fn read<'a>(&'a mut self) -> Result<u8, OFileError<'a>> {
        let reader: &mut BufReader<File> = match &mut self.mode {
            OFileMode::Read(r) => r,
            OFileMode::Modify(r, _) => r,
            OFileMode::Write(_) => return Err(OFileError::CannotReadFile(&self.file_path))
        };
        
        let mut bytes = [0u8];
        // Trys to read bytes, if reaches end of stream, throws erorr
        if unwrap_result!(reader.read(&mut bytes) => |e| Err(OFileError::IOError(e))) == 0usize {
            return Err(OFileError::EndOfStream);
        };

        self.current = Some(bytes[0]);
        Ok(bytes[0])
    }

    pub fn write<'a>(&'a mut self, value: u8) -> Result<(), OFileError<'a>> {
        let writer: &mut BufWriter<File> = match &mut self.mode {
            OFileMode::Write(w) => w,
            OFileMode::Modify(_, w) => w,
            OFileMode::Read(_) => {
                self.mode = OFileMode::modify_from_read(&self.file_path, &self.idx)?;
                if let OFileMode::Modify(_, w) = &mut self.mode {
                    w
                } else { panic!("shouldn't logically panic") }
            },
        };

        unwrap_result!(writer.write(&[value]) => |e| Err(OFileError::IOError(e)));
        Ok(())
    }
}