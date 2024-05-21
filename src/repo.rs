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

// TO-DO
// Add check to make sure valid path is also a valid Repo
impl Repo {
    pub async fn new(input_path: &str) -> Option<Self> {
        if input_path.is_empty() {
            return None;
        }

        let path = PathBuf::from(input_path);
        let name = path.file_name().unwrap().to_str().map(|s| s.to_string())?;
        if path.exists() {
            return Some(Repo { name, path });
        }

        None
    }
}

// TO-DO
// The lookup map is improperly implemented
pub struct RepoList {
    pub repos: Vec<Repo>,
    pub lookup: HashMap<String, String>,
}

impl RepoList {
    pub fn new() -> Self {
        RepoList {
            repos: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    /* Acts like a check when appending paths to config_file, although this isn't the most efficent
     * way since a repo is created and discarded */
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

    // TO-DO
    // Make The list prettier
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

    // Asynchronously updates every Repo
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
                    Err(e) => println!("{}{}{}", "Failed To Update ".red(), repo.name.red(), e),
                };
            }));
        }

        join_all(tasks).await;
    }
}
