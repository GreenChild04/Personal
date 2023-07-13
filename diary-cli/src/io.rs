use crate::error;
use std::fs;

pub fn mkdir(path: &str) {
    error::try_do(
        "IO", 
        "Failed to create dir", 
        &format!("Failed to create dir '{path}'"), 
        3, ||
        fs::create_dir_all(path),
    );
}

pub fn write_to_file(path: &str, text: &str) {
    error::try_do(
        "IO", 
        "Failed to write to file", 
        &format!("Failed to write to file '{path}'"), 
        3, || 
        fs::write(path, text),
    );
}

pub fn read_file_to_string(path: &str) -> String {
    error::try_do(
        "IO",
        "Failed to read from file",
        &format!("Failed to read string from file '{path}'"),
        3, ||
        fs::read_to_string(path),
    )
}