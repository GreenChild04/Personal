use crate::version::Version;

pub struct BuildConfig;

impl BuildConfig {
    pub const VERSION_STRING: &str = "0.1.0";
    pub const VERSION: Version = Version::new(0, 1, 0);
    pub const AUTHOR: &str = "GreenChild04";
    pub const ABOUT: &str = "A minimalist and user-friendly command-line interface (CLI) tool written in rust for keeping a diary.";
    pub const INIT_FILE: &str = "diary-cli.init";
    pub const DIRS: [&str; 2] = ["archive", "tmp"];
}