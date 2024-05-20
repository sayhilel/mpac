use crate::git::_pull;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use futures::Future;
use std::pin::Pin;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::Semaphore;
use tokio::task;
use tokio::task::JoinHandle;
#[derive(Clone, Debug)]
pub struct Repo {
    pub name: String,
    pub path: PathBuf,
}

impl Repo {
    pub async fn new(path_: &str) -> Option<Self> {
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
    pub async fn add_repo(&mut self, path: &str) -> Result<()> {
        if let Some(repo) = Repo::new(path).await {
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
    pub async fn update_all(&self) {
        if self.repos.is_empty() {
            println!("NO REPOS");
        }
        let mut tasks = vec![];
        for repo in self.repos.iter().cloned() {
            tasks.push(task::spawn(async move {
                match _pull(&repo).await {
                    Ok(()) => println!("Succesfully update {}", repo.name),
                    Err(e) => println!("{}", e),
                };
            }));
        }

        // Await all tasks concurrently
        join_all(tasks).await;
    }
}
