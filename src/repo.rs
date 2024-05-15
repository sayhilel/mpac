use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    io::{self, prelude, BufRead, BufReader, Error},
};

// CHANGE-ME
// Keep track of a repo's path and last updated time
pub struct Repo {
    pub path: String,
    pub time: usize,
}

// Implement functions for repos here

pub fn load_repos() -> Result<Vec<Repo>, Error> {
    // Open the file
    let path = Path::new("/home/croxymoc/Documents/Projects/manpac/target/debug/repos.txt"); // Very system dependant // Remember to change
    let repo_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    // Load ; REPLACE WITH JSON LATER
    let mut repos = vec![]; // Mutable vector to store the repo objects
    let mut reader = BufReader::new(repo_file);

    // Very ugly and impractical but we ball for now
    for repo_path in reader.lines() {
        repos.push(Repo {
            path: repo_path?,
            time: 1,
        });
    }

    Ok(repos)
}

// update_repos
