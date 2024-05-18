use crate::repo::Repo;

use std::error::Error;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct RepoList {
    pub repos: Vec<Repo>,
}

impl RepoList {
    pub fn new(config_path: &Path) -> Result<RepoList, Box<dyn Error>> {
        let repo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path)?;

        let mut reader = BufReader::new(repo_file);
        let mut repo_list = RepoList { repos: vec![] };

        reader.lines().for_each(|x| match x {
            Ok(mut path) => {
                let (valid, repo_path) = Repo::new(&mut path);
                if (valid) {
                    repo_list.repos.push(repo_path);
                }
            }
            Err(err) => panic!("{}", err),
        });

        Ok(repo_list)
    }

    pub fn list_repos(&self) {
        if self.repos.is_empty() {
            println!("NO REPOS");
        }
        self.repos.iter().enumerate().for_each(|(index, repo)| {
            println!("{}: {}", index, repo.path);
        });
    }

    pub fn update_repos(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
