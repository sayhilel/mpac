use std::path::{Path, PathBuf};

use std::ffi::OsStr;
// A hashmap for this lookup would be great

pub struct Repo {
    pub name: String,
    pub path: PathBuf,
}

impl Repo {
    pub fn new(path_: &str) -> Option<Self> {
        if path_.is_empty() {
            return None;
        }

        let path = PathBuf::from(path_);

        if path.exists() {
            let name = path.file_name().unwrap().to_str().map(|s| s.to_string())?;
            Some(Repo { name, path })
        } else {
            println!("Path '{}' Doesn't exist. Check the config file.", path_);
            None
        }
    }
}
