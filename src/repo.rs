use std::path::{Path, PathBuf};

use std::ffi::OsStr;
// A hashmap for this lookup would be great
pub fn get_name(path: &Path) -> Option<String> {
    path.file_name().unwrap().to_str().map(|s| s.to_string())
}

pub struct Repo {
    pub name: String,
    pub path: PathBuf,
}

impl Repo {
    //
    pub fn new(path_: &String) -> Option<Repo> {
        let path = PathBuf::from(path_.trim());
        let name = path.file_name().unwrap().to_str().map(|s| s.to_string())?;

        // Validate the path
        if path.exists() && !name.is_empty() {
            Some(Repo { name, path })
        } else {
            None
        }
    }
    //
}
