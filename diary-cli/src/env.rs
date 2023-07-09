use std::{
    path::Path,
    fs,
};
use crate::{
    error,
    build_config::BuildConfig,
    version::Version,
};

pub struct Environment {
    path: String,
    archive: String,
    tmp: String,
}

impl Environment {
    fn new(path: &str) -> Self { // Dangerous function; trusts that the env is valid
        Self {
            path: path.to_string(),
            archive: format!("{path}/{}", BuildConfig::DIRS[0]),
            tmp: format!("{path}/{}", BuildConfig::DIRS[1]),
        }
    }

    pub fn load_or_init(raw_path: &str) -> Self {
        let path: String = format!("{raw_path}/{}", BuildConfig::INIT_FILE);
        let path: &Path = Path::new(&path);
        if path.exists() { Self::load(raw_path) }
        else { Self::init(raw_path) }
    }

    pub fn init(raw_path: &str) -> Self {
        // Checks for env directory, if not it creates it
        let path: &Path = Path::new(raw_path);
        if !path.exists() { fs::create_dir_all(path).expect("Error: Could not create directories") }

        // Write init file
        fs::write(format!("{raw_path}/{}", BuildConfig::INIT_FILE), BuildConfig::VERSION_STRING).expect("Error: Couldn't create init file");

        // Create dirs
        BuildConfig::DIRS.iter().for_each(|x| fs::create_dir_all(format!("{raw_path}/{x}")).expect("Error: Couldn't create directory"));

        Self::new(raw_path)
    }

    pub fn load(raw_path: &str) -> Self {
        // Checks for env path
        let path = Path::new(raw_path);
        error::init::<()>("Environment", "Path not found", &format!("Path '{raw_path}' not found!"))
            .true_or_throw(path.exists());

        // Checks for init file
        let init_path: String = format!("{raw_path}/{}", BuildConfig::INIT_FILE);
        let path: &Path = Path::new(&init_path);
        error::init::<()>("Environment", "File not found", &format!("Diary CLI init file not found in '{raw_path}'"))
            .true_or_throw(path.exists());
        Self::check_init_version(&fs::read_to_string(path).expect("Error: Couldn't read init file"), &init_path);
        fs::write(path, BuildConfig::VERSION_STRING).expect("Error: Couldn't write to init file"); // Replace init version with current

        Self::check_for_dirs(raw_path, &BuildConfig::DIRS); // Checks for directories

        // Loads Environment
        Self::new(raw_path)
    }

    fn check_init_version(version: &str, path: &str) {
        let version: Version = Version::parse(version)
            .unwrap_or_else(|_| error::init("Environemnt", "Invalid init file", &format!("'{0}' is not a valid version ({1})", version, path)).crash());
        // Checks version compatibility
        error::init::<()>("Enivronment", "Incompatible version", &format!("Version '{0}' found in environment is incompatible with current version '{1}'", &version, &BuildConfig::VERSION))
            .true_or_throw(BuildConfig::VERSION.is_compatible(version));
    }

    fn check_for_dir(raw_path: String) {
        let path: &Path = Path::new(&raw_path);
        error::init::<()>("Environment", "Directory not found", &format!("Directory '{raw_path}' not found"))
            .true_or_throw(path.exists());
    }

    fn check_for_dirs(raw_path: &str, dirs: &[&str]) {
        dirs.iter().for_each(|x| Self::check_for_dir(format!("{raw_path}/{x}")));
    }
}