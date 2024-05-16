use git2::Repository;

use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    io::{self, prelude, BufRead, BufReader, Error},
};

// Might help later
pub struct Repos {
    repos: Vec<Repository>,
}

// Implement functions for repos here
pub fn new() -> Result<Repos, Box<dyn std::error::Error>> {
    // Open the file
    let path = Path::new("/home/croxymoc/Documents/Projects/manpac/target/debug/repos.txt"); // Very system dependant // Remember to change
    let repo_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    // Load ; REPLACE WITH JSON LATER
    let mut reader = BufReader::new(repo_file);
    let mut repos_new = Repos { repos: vec![] };
    // IDK how to do this with map
    for repo_path in reader.lines() {
        repos_new.repos.push(Repository::open(repo_path?)?);
    }

    Ok(repos_new)
}

//
impl Repos {
    pub fn list_repos(&self) {
        match self.repos.len() {
            0 => print!("NO REPOS --CHANGE-ME\n"),
            _ => {
                for repo in self.repos.iter() {
                    // No error check for this unwrap :(
                    print!("{}\n", repo.path().to_str().unwrap());
                }
            }
        }
    }

    // Just gonna update all repos for now
    pub fn update_repos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
