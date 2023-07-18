use rand::Rng;
use self::error::LockError;
use std::path::Path;
use crate::*;

pub const FILE_EXTENSION: &str = ".lock";

pub struct Lock(String); // Holds path

impl Drop for Lock {
    fn drop(&mut self) {
        // log("Lock", "Unlocking", &format!("Unlocking lock '{}'...", self.0));
        if !file_exists(&self.0) { return; }
        if std::fs::remove_file(&self.0).is_ok() {} // Do nothing if fails to remove lock-file
    }
}

#[inline(always)]
pub fn file_exists(path: &str) -> bool { Path::new(path).is_file() }

#[inline]
pub fn check_lock_path(path: &str) -> Result<(), LockError> {
    // Checks if lock path is valid / exists even
    if !bind!(Path::new(&path) => |this: &Path| this.exists() && this.is_dir()) {
        return Err(LockError::InvalidLockPath(path.to_string()))
    }

    // Checks for any other lock files
    let entries = std::fs::read_dir(path).map_err(LockError::IOError)?;
    let is_locks: bool = entries
        .filter_map(|entry| entry.ok())
        .any(|entry| {
            entry.file_name()
                .to_string_lossy()
                .ends_with(FILE_EXTENSION)
        });
    if is_locks { return Err(LockError::AnotherLockFile) };

    Ok(()) // if everything passes
}

/// for usage in diary-cli
#[macro_export]
macro_rules! lock {
    ($new_fn:path, $path:expr; $($args:expr),*) => {
        $new_fn($($args),*, lock: $crate::lock::gen_lock($path))
    };
}

// for usage in diary-cli
pub fn gen_lock(path: String) -> Lock {
    use std::{thread::sleep, time::Duration};

    let res = lock(&path);
    use LockError::*;
    if let Err(e) = &res {
        match e {
            InvalidLockPath(p) => crate::error::crash("Lock Failure", "Failed to find lock directory", &format!("Path '{p}' not found or invalid")),
            IOError(e) => crate::error::crash("IO", "IO Error", &e.to_string()),
            AnotherLockFile => { // Wait for 500ms, try again; repeat
                log("Lock", "Other lock files found", "Waiting for other process to end...");
                // Loop until there are no errors
                loop {
                    sleep(Duration::from_millis(500));
                    if let Ok(o) = lock(&path) { return o }
                }
            },
        }
    }

    else { res.unwrap() }
}

pub fn lock(path: &str) -> Result<Lock, LockError> { // Safe interface for locking
    check_lock_path(path)?;
    init_lock(path)
}

#[inline]
fn init_lock(path: &str) -> Result<Lock, LockError> { // trusts that lock path is valid and that there are not other locks
    let uid: u8 = rand::thread_rng().gen();
    // Writing to the lock file
    // Eg: `/home/1234.lock`
    let lock_file: String = format!("{path}/{uid}{}", FILE_EXTENSION);
    if let Err(e) = std::fs::write(&lock_file, "lock files; to prevent race conditions -GreenChild 2023") {
        return Err(LockError::IOError(e));
    }

    // For dropping purposes
    Ok(Lock(lock_file))
}

pub mod error {
    use std::error::Error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub enum LockError {
        InvalidLockPath(String),
        IOError(std::io::Error),
        AnotherLockFile,
    }

    impl Display for LockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::InvalidLockPath(p) => write!(f, "Invalid lock path '{p}'"),
                Self::IOError(e) => write!(f, "IO Error: {:?}", e),
                Self::AnotherLockFile => write!(f, "Other lock file found"),
            }
        }
    }

    impl Error for LockError {}
}