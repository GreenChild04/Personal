use crate::prelude::*;
use std::path::Path;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};

pub enum OFileMode {
    Read,
    Write,
    Modify,
}

enum OFileModeInternal {
    Read(BufReader<File>),
    Write(BufWriter<File>),
    Modify(BufReader<File>, BufWriter<File>),
}

impl OFileModeInternal {
    pub fn from(mode: OFileMode, file: File) -> Result<OFileModeInternal, CapError> {
        todo!();
    }
}

pub struct OFile {
    file: File,
    mode: OFileModeInternal,
    idx: u64,
    current: Option<u8>,
}

macro_rules! unwrap_result {
    ($result:expr => $wrapper:expr) => {{
        let result = $result;
        if let Err(e) = result {
            return $wrapper(e);
        } result.unwrap()
    }}
}

impl OFile {
    pub fn new(file_path: &str, mode: OFileMode) -> Result<Self, CapError> {
        let file = match mode {
            OFileMode::Write => unwrap_result!(File::create(file_path) => |e| Err(CapError::IOError(e))),
            OFileMode::Read => {
                if !Path::new(file_path).exists() { return Err(CapError::FileNotFound(file_path.to_string())) }
                unwrap_result!(File::open(file_path) => |e| Err(CapError::IOError(e)))
            },
            OFileMode::Modify => todo!(),
        };

        todo!()
    }
}

