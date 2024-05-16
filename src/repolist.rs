use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

// TO-DO: Add input validation
pub struct RepoPath {
    pub path: String,
}

impl RepoPath {
    pub fn new(path: &mut String) -> (bool, RepoPath) {
        (
            !path.trim().is_empty(),
            RepoPath {
                path: path.trim().to_string(),
            },
        )
    }
}

pub struct RepoList {
    pub repos: Vec<RepoPath>,
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
                let (valid, repo_path) = RepoPath::new(&mut path);
                if (valid) {
                    repo_list.repos.push(repo_path);
                }
            }
            Err(err) => panic!("{}", err),
        });

        Ok(repo_list)
    }

    pub fn list_repos(&self) {
        self.repos.iter().enumerate().for_each(|(index, repo)| {
            println!("{}: {}", index, repo.path);
        });
    }

    pub fn update_repos(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
