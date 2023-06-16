mod isol;
use diary_cli::env::Environment;
use isol::*;
use std::{fs};

#[test]
#[should_panic(expected="File not found")]
fn isol_env_load_path() {
    let tmp_path: TmpPath = isol::new_env();
    Environment::load(tmp_path.path());
}

#[test]
#[should_panic(expected="Directory '")]
fn isol_env_load_no_dir() {
    let tmp_path: TmpPath = isol::new_env();
    fs::write(format!("{0}/diary-cli.init", tmp_path.path()), "").unwrap();
    Environment::load(tmp_path.path());
}