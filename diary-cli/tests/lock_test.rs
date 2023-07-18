mod isol;
use isol::*;
use diary_cli::lock::*;
use diary_cli::lock::error::*;

#[test]
fn isol_lock_invalid_path() {
    let tmp_path = new_env();
    let path = format!("{}/doesn't_exist", tmp_path);
    let res = lock(&path);
    if let Err(LockError::InvalidLockPath(_)) = res {}
    else { panic!("Expected LockError 'InvalidLockPath'!") }
}

#[test]
fn isol_lock_another_lock() {
    let tmp_path = new_env();
    let _first_lock = lock(&tmp_path.to_string()).unwrap();
    let res = lock(&tmp_path.to_string());
    if let Err(LockError::AnotherLockFile) = res {}
    else { panic!("Expected Lock Error 'AnotherLockFile'!") }
}