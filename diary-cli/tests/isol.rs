use std::fs;
use std::path::Path;
use rand::{thread_rng, Rng};
use std::fmt::Display;
use std::fmt;

pub fn new_env<'a>() -> TmpPath {
    // creates 'test_tmp' folder if it doesn't exist
    let path = Path::new("./test_tmp/");
    if !path.exists() {
        fs::create_dir_all(path).expect("Error: couldn't create directory!");
    } let random: String = thread_rng().gen_range(1000..10000).to_string();
    return TmpPath::new(format!("./test_tmp/{random}"));
}

pub struct TmpPath(String);

impl TmpPath {
    pub fn new(path: String) -> Self {
        let path_link = Path::new(&path);
        if !path_link.exists() {
            match fs::create_dir_all(path_link) {
                Ok(_) => (),
                Err(e) => panic!("Error: creating directory: {:?}", e),
            }
        } return Self(path);
    }
}

impl Display for TmpPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  &self.0)
    }
}

impl Drop for TmpPath {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("Error: Could not delete tmp directory");
        std::mem::drop(self);
    }
}