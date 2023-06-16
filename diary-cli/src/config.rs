use crate::version::Version;

pub struct Config;

impl Config {
    pub const VERSION: Version = Version::new(0, 1, 0);
    pub const VERSION_STRING: String = Self::VERSION.to_string();
    pub const AUTHOR: &str = "GreenChild04";
    pub const ABOUT: &str = "A minimalist and user-friendly command-line interface (CLI) tool written in rust for keeping a diary.";
}