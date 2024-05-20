use crate::git::_pull;
use anyhow::{anyhow, Result};
use std::{collections::HashMap, path::PathBuf};

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

pub struct RepoList {
    pub repos: Vec<Repo>,
    pub lookup: HashMap<String, String>,
}

impl RepoList {
    // Constructor
    pub fn new() -> Self {
        RepoList {
            repos: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    // Bool result from adding a repo to a repolist
    pub fn add_repo(&mut self, path: &str) -> Result<()> {
        if let Some(repo) = Repo::new(path) {
            // TO-DO use lifetimes later
            self.lookup.insert(path.to_string(), repo.name.clone());
            self.repos.push(repo);
            return Ok(());
        }
        Err(anyhow!("Couldn't add repo."))
    }

    // List all repos in the repolist
    pub fn list(&self) {
        if self.repos.is_empty() {
            println!("NO REPOS");
        }
        self.repos.iter().enumerate().for_each(|(index, repo)| {
            println!("{}:{}", index, repo.name);
        });
    }

    // Performs a git pull on all repos
    // TO-DO use async
    // switch to git2
    pub fn update_all(&self) {
        if self.repos.is_empty() {
            println!("NO REPOS");
        }
        self.repos.iter().enumerate().for_each(|(index, repo)| {
            println!("updating {}", &repo.name);
            _pull(&repo);
        });
    }
}
