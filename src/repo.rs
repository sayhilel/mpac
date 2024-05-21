use crate::git::_pull;
use anyhow::{anyhow, Result};
use colored::Colorize;
use futures::future::join_all;
use std::{collections::HashMap, path::PathBuf};
use tokio::task;

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
        match Repo::new(path).await {
            Some(repo) => {
                self.lookup.insert(path.to_string(), repo.name.clone());
                self.repos.push(repo);
                return Ok(());
            }
            None => Err(anyhow!(
                "{}{}{}",
                "Unable to add repo. Make sure the path \"".red(),
                path.yellow(),
                "\" is valid".red()
            )),
        }
    }

    // List all repos in the repolist
    pub fn list(&self) {
        if self.repos.is_empty() {
            println!(
                "{}\n{}",
                "No repos found in config file. Try adding a repo with ".red(),
                "\"$ mpac add --repo <path>\"".yellow()
            );
        }
        self.repos.iter().enumerate().for_each(|(index, repo)| {
            println!("{}:{}", index + 1, repo.name.green());
        });
    }

    // Performs a git pull on all repos
    // switch to git2
    pub async fn update_all(&self) {
        if self.repos.is_empty() {
            println!(
                "{}\n{}",
                "No repos found in config file.".red(),
                "Try adding a repo with \"mpac add --repo <path>\"".yellow()
            );
        }
        let mut tasks = vec![];
        for repo in self.repos.iter().cloned() {
            tasks.push(task::spawn(async move {
                match _pull(&repo).await {
                    Ok(()) => println!("{}: {}", "Updated".green(), repo.name.green()),
                    Err(e) => println!("{}{}", "Failed to update due to ".red(), e),
                };
            }));
        }

        // Await all tasks concurrently
        join_all(tasks).await;
    }
}
