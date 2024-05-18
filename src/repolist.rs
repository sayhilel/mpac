use crate::repo::Repo;

use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Error},
    path::Path,
};

pub struct RepoList {
    pub repos: Vec<Repo>,
}

impl RepoList {
    pub fn new() -> Self {
        RepoList { repos: Vec::new() }
    }

    pub fn load(&mut self, config_path: &Path) -> Result<(), Error> {
        let repo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path)?;

        let mut reader = BufReader::new(repo_file);

        for lines in reader.lines() {
            if let Ok(path) = lines {
                if path.trim().is_empty() {
                    continue;
                }
                match Repo::new(&path) {
                    Some(repo) => self.repos.push(repo),
                    None => println!("Path is not valid: {}", path),
                }
            } else {
                println!("Couldn't read line");
            }
        }

        Ok(())
    }

    pub fn list_repos(&self) {
        if self.repos.is_empty() {
            println!("NO REPOS");
        }
        self.repos.iter().enumerate().for_each(|(index, repo)| {
            println!("{}:", index);
        });
    }
}
