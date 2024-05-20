use crate::repo::{Repo, RepoList};
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dirs::config_dir;
use std::io::prelude::*;

use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, BufWriter},
    path::Path,
};

pub struct Config {
    pub file: String,
}

// TODO add validation and custom config files
impl Config {
    // Private

    // Public
    pub fn default() -> Self {
        let mut config_ = config_dir()
            .expect("Unable to locate config director")
            .to_str()
            .expect("Unable to resolve")
            .to_string();
        config_ += "/mpac.conf";
        println!("{}", config_);
        Self { file: config_ }
    }

    pub async fn load(&self, repo_list: &mut RepoList) -> Result<()> {
        let repo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)
            .expect("Unable to create config file");

        let mut reader = BufReader::new(repo_file);

        for lines in reader.lines() {
            if let Ok(path) = lines {
                repo_list.add_repo(&path.trim()).await?
            } else {
                println!("Unable to read file")
            }
        }

        Ok(())
    }

    pub async fn add_to_file(&self, repo_list: &mut RepoList, ipath: &String) -> Result<()> {
        let check = &ipath.clone();

        if repo_list.lookup.contains_key(check.trim()) {
            return Err(anyhow!("Path already exists"));
        }

        repo_list.add_repo(ipath).await?;
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file)
            .expect("Unable to write");

        writeln!(file, "{}", ipath)?;

        Ok(())
    }
}
