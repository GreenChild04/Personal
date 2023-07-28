use std::fs::File;
use std::io::{BufReader, BufWriter};
use super::*;

pub enum OFileMode {
    Read,
    Write,
    Modify,
}

pub(super) enum OFileModeInternal {
    Read(BufReader<File>, u8),
    Write(BufWriter<File>),
    Modify(BufReader<File>, BufWriter<File>),
}

impl OFileModeInternal {
    pub fn from(mode: OFileMode, file: File, file_path: &str) -> Result<OFileModeInternal, CapError> {
        match mode {
            OFileMode::Read => Ok(OFileModeInternal::Read(BufReader::new(file), 0u8)),
            OFileMode::Write => Ok(OFileModeInternal::Write(BufWriter::new(file))),
            OFileMode::Modify => {
                let reader = BufReader::new(file);
                let writer = BufWriter::new(unwrap_result!(File::create(format!("{file_path}.new")) => |e| Err(CapError::IOError(e))));
                Ok(OFileModeInternal::Modify(
                    reader,
                    writer,
                ))
            },
        }
    }
}