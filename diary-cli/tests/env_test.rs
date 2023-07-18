mod isol;
use diary_cli::env::Environment;
use diary_cli::build_config::BuildConfig;
use isol::*;
use std::fs;

#[test]
#[should_panic(expected="File not found")]
fn isol_env_load_path() {
    let tmp_path: TmpPath = isol::new_env();
    Environment::load(&tmp_path.to_string());
}

#[test]
#[should_panic(expected="Incompatible version")]
fn isol_env_load_invalid_init() {
    let tmp_path: TmpPath = isol::new_env();
    fs::write(format!("{0}/diary-cli.init", tmp_path), "7.1.23").unwrap();
    Environment::load(&tmp_path.to_string());
}

#[test]
#[should_panic(expected="Directory not found")]
fn isol_env_load_missing_dir() {
    let tmp_path: TmpPath = isol::new_env();
    fs::write(format!("{0}/diary-cli.init", tmp_path), BuildConfig::VERSION_STRING).unwrap();
    Environment::load(&tmp_path.to_string());
}

#[test]
fn isol_env_init_load() {
    let tmp_path: TmpPath = isol::new_env();
    Environment::init(&tmp_path.to_string());
    Environment::load(&tmp_path.to_string());
}