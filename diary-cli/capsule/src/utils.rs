pub mod ofile;
pub use ofile::*;
pub use crate::unwrap_result;

use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use tar::Builder;
use zstd::stream::Encoder;
use crate::error::*;
use walkdir::WalkDir;

#[macro_export]
macro_rules! unwrap_result {
    ($result:expr => $wrapper:expr) => {{
        let result = $result;
        if let Err(e) = result {
            return $wrapper(e);
        } result.unwrap()
    }}
}

pub fn compress_file<'a>(file_path: &'a str, out_path: &str, compression_level: i32, buff_size: usize, handler: impl CapErrHandler) {
    let handler = ErrHandler::new(handler, CapErrContext::WhileZippingFile(file_path));

    let mut file = handle!((handler) File::open(file_path) => |e| CapError::IOError(e));
    let out_file = handle!((handler) File::create(out_path) => |e| CapError::IOError(e));

    let mut encoder = handle!((handler) Encoder::new(&out_file, compression_level) => |e| CapError::IOError(e));
    let _ = encoder.set_parameter(zstd::zstd_safe::CParameter::WindowLog(buff_size as u32));

    let mut buffer = vec![0u8; buff_size];
    loop {
        let bytes_read = handle!((handler) (file.read(&mut buffer)) => |e| CapError::IOError(e));
        if bytes_read == 0 { break }
        handle!((handler) (encoder.write_all(&buffer[..bytes_read])) => |e| CapError::IOError(e));
    }
    handle!((handler) (encoder.finish()) => |e| CapError::IOError(e));
}

pub fn build_tarball<'a>(path: &str, out_path: &str, handler: impl CapErrHandler) {
    let handler = ErrHandler::new(handler, CapErrContext::WhileBuildingTarBall(path));

    let tarball = handle!((handler) File::create(out_path) => |e| CapError::IOError(e));
    let mut builder = Builder::new(tarball);

    for entry in WalkDir::new(path) {
        let entry = handle!((handler) (entry) => |e| CapError::WalkDirError(e));
        if entry.file_type().is_dir() {
            continue;
        } handle!((handler) (builder.append_path(entry.path())) => |e| CapError::IOError(e));
    }
}

pub fn compress(string_path: &str, out_path: &str, compression_level: i32, buff_size: usize, raw_handler: impl CapErrHandler + Copy) {
    let handler = ErrHandler::new(raw_handler, CapErrContext::WhileZipping(string_path));

    let path = Path::new(string_path);
    if !path.exists() {
        return handler.runtime(CapError::FileNotFound(string_path.to_string()), None);
    }

    if path.is_file() {
        return compress_file(string_path, out_path, compression_level, buff_size, raw_handler);
    }
}