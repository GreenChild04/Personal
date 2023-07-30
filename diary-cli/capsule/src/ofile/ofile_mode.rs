use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use super::*;

pub enum OFileMode {
    Read(BufReader<File>),
    Write(BufWriter<File>),
    Modify(BufReader<File>, BufWriter<File>),
}

impl OFileMode {
    #[inline]
    pub(super) fn new_read(file: File) -> OFileMode {
        OFileMode::Read(BufReader::new(file))
    }

    #[inline]
    pub(super) fn new_write(file: File) -> OFileMode {
        OFileMode::Write(BufWriter::new(file))
    }

    pub(super) fn modify_from_read<'a>(read_file_path: &str, idx: &u64) -> Result<OFileMode, OFileError<'a>> {
        let read_file = unwrap_result!(File::open(read_file_path) => |e| Err(OFileError::IOError(e)));
        let mut reader = BufReader::new(read_file);
        let mut writer = BufWriter::new( unwrap_result!(File::create(format!("{read_file_path}.new")) => |e| Err(OFileError::IOError(e))) );
        
        for _ in 0..*idx {
            let mut read = [0u8];
            let bytes_read = unwrap_result!(reader.read(&mut read) => |e| Err(OFileError::IOError(e)));
            unwrap_result!(writer.write(&read[0..bytes_read]) => |e| Err(OFileError::IOError(e)));
        }
        
        Ok(OFileMode::Modify(reader, writer))
    }
}