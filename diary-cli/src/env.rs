use std::path::Path;
use crate::{
    *,
    build_config::BuildConfig,
    version::Version,
    io::*,
};

pub struct Environment {
    path: String,
    archive: String,
    tmp: String,
}

impl Environment {
    fn unsafe_load(path: &str) -> Self { // Dangerous function; trusts that the env is valid
        Self {
            path: path.to_string(),
            archive: format!("{path}/{}", BuildConfig::DIRS[0]),
            tmp: format!("{path}/{}", BuildConfig::DIRS[1]),
        }
    }

    pub fn load_or_init(path: &str) -> Self {
        niceif! {
            Path::new(&format!("{path}/{}", BuildConfig::INIT_FILE)).exists(),
            Self::load(path),
            Self::init(path),
        }
    }

    pub fn init(path: &str) -> Self {
        // Checks for env directory, if not it creates it
        if !Path::new(path).exists() { mkdir(path) }

        // Write init file
        let init_file: String = format!("{path}/{}", BuildConfig::INIT_FILE);
        write_to_file(&init_file, BuildConfig::VERSION_STRING);

        // Create dirs
        BuildConfig::DIRS.iter().for_each(|x| mkdir(&format!("{path}/{x}")));

        Self::unsafe_load(path)
    }

    pub fn load(path: &str) -> Self {
        // Checks for env path
        true_or_throw! {
            "Environment",
            "Path not found",
            format!("Path '{path}' not found!"),
            Path::new(path).exists();
        };

        // Checks for init file
        let init_path: String = format!("{path}/{}", BuildConfig::INIT_FILE);
        true_or_throw! {
            "Environment",
            "File not found",
            format!("Diary CLI init file not found in '{path}'"),
            Path::new(&init_path).exists();
        };
        Self::check_init_version(&read_file_to_string(&init_path), &init_path);
        write_to_file(&init_path, BuildConfig::VERSION_STRING); // Overwrites init file

        Self::check_for_dirs(path, &BuildConfig::DIRS); // Checks for directories

        // Loads Environment
        Self::unsafe_load(path)
    }

    fn check_init_version(version: &str, path: &str) {
        let version: Version = Version::parse(version)
            .unwrap_or_else(|_| error::init("Environemnt", "Invalid init file", &format!("'{0}' is not a valid version ({1})", version, path)).crash());
        // Checks version compatibility
        true_or_throw! {
            "Environment",
            "Incompatible version",
            format!("Version '{0}' found in environment is incompatible with current version '{1}'", &version, &BuildConfig::VERSION),
            BuildConfig::VERSION.is_compatible(version);
        };
    }

    fn check_for_dir(path: &str) {
        true_or_throw! {
            "Environment",
            "Directory not found",
            format!("Directory '{path}' not found"),
            Path::new(path).exists();
        }
    }

    fn check_for_dirs(path: &str, dirs: &[&str]) {
        dirs.iter().for_each(|x| Self::check_for_dir(&format!("{path}/{x}")));
    }
}