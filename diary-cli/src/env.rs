use std::{
    path::Path,
    fs,
};
use crate::error::Error;

pub struct Environment {
    path: String,
}

impl Environment {
    pub fn init(raw_path: &String) {
        // Checks for env directory, if not it creates it
        let path: &Path = Path::new(raw_path);
        if !path.exists() { fs::create_dir_all(path).expect("Error: Could not create directories") }
    }

    pub fn load(raw_path: &String) -> Environment {
        // Checks for env path
        let path = Path::new(raw_path);
        Error::new("Environment", "Path not found", format!("Path '{raw_path}' not found!"))
            .true_or_throw(path.exists());

        // Checks for init file
        let init_path: String = format!("{raw_path}/diary-cli.init");
        let path: &Path = Path::new(&init_path);
        Error::new("Environment", "File not found", format!("Diary CLI init file not found in '{raw_path}'"))
            .true_or_throw(path.exists());

        Self::check_for_dirs(raw_path, &["archive", "tmp"]); // Checks for directories

        // Loads Environment
        Environment {
            path: raw_path.clone()
        }
    }

    fn check_for_dir(raw_path: String) {
        let path: &Path = Path::new(&raw_path);
        Error::new("Environment", "Directory not found", format!("Directory '{raw_path}' not found"))
            .true_or_throw(path.exists());
    }

    fn check_for_dirs(raw_path: &String, dirs: &[&str]) {
        dirs.iter().for_each(|x| Self::check_for_dir(format!("{raw_path}/{x}")));
    }
}