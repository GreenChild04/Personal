use super::error::Error;
use std::fs;

pub struct Args {
    env: String,

}

pub fn parse_args(args: Vec<String>) {
    if args.len() < 2 { Error::throw("Diary Cli", "Too few arguments", "Expected at least 1 argument"); }
    parse_env(&args[1]);
}

fn parse_env(env: &str) {
    if let Ok(metadata) = fs::metadata(env) {
        if metadata.is_dir() {
            return ();
        } else {
            Error::throw("IO", "Not a valid directory", "Was given the path of a file not of a directory");
        }
    } else {
        Error::throw("IO", "Path not found", &format!("Path [{env}] doesn't exist"));
    }
}