use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    io::{self, prelude, BufRead, BufReader, Error},
};

// CHANGE-ME
// Keep track of a repo's path and last updated time
pub struct Repo {
    pub path: String,
    //pub time: usize,
}

// Might help later
pub struct Repos {
    repos: Vec<Repo>,
}

// Implement functions for repos here
pub fn new() -> Result<Repos, Error> {
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
    // Very ugly and impractical but we ball for now
    for repo_path in reader.lines() {
        repos_new.repos.push(Repo { path: repo_path? });
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
                    print!("{}\n", repo.path);
                }
            }
        }
    }
}
//pub fn update_repos(repo) -> Result<(), Error> {}
