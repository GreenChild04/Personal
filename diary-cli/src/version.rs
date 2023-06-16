pub mod error;
pub use error::*;
use std::fmt;

#[derive(Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Version {
    pub const fn new(major: u8, minor: u8, build: u8) -> Self {
        Self {
            major,
            minor,
            build,
        }
    }

    pub fn parse(version: &str) -> Result<Self, VersionError> {
        let parts: Vec<&str> = version.split('.').collect();

        // Checks separator count
        if parts.len() != 3 {
            return Err(VersionError::InvalidSeparator(format!("Expected 2 '.' separators within version, got {}", parts.len() - 1)));
        }
        
        // Checks for valid numbers
        let major = parts[0].parse::<u8>().map_err(|_| VersionError::InvalidVersion)?;
        let minor = parts[1].parse::<u8>().map_err(|_| VersionError::InvalidVersion)?;
        let build = parts[2].parse::<u8>().map_err(|_| VersionError::InvalidVersion)?;
        
        // Builds version
        Ok(Version::new(major, minor, build))
    }

    pub fn is_compatible(&self, other: Self) -> bool {
        other.major == self.major && other.minor <= self.minor
    }

    pub fn is_compatible_or_else<F: FnOnce()>(&self, other: Self, f: F) {
        if !self.is_compatible(other) { f() }
    }

    pub fn to_string(&self) -> String {
        format!("{0}.{1}.{2}", self.major, self.minor, self.build)
    }
}