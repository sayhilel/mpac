use crate::repo::{Repo, RepoList};
use anyhow::{self, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dirs::config_dir;
use std::io::prelude::*;

use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Error},
    path::Path,
};

pub struct Config {
    pub file: String,
}

// TODO add validation and custom config files
impl Config {
    pub fn default() -> Self {
        let config_ = config_dir().expect("Unable to locate config director");

        Self {
            file: config_.to_str().expect("Unable to unwrap").to_string() + "mpac.conf",
        }
    }

    pub fn load(&self, repo_list: &mut RepoList) -> Result<(), Error> {
        let repo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)
            .expect("Unable to create config file");

        let mut reader = BufReader::new(repo_file);

        for lines in reader.lines() {
            if let Ok(path) = lines {
                if let Some(repo) = Repo::new(path.trim()) {
                    repo_list.repos.push(repo);
                }
            } else {
                println!("Unable to read file")
            }
        }

        Ok(())
    }

    pub fn add_to_file(&self, repo_list: &mut RepoList, ipath: &String) -> Result<()> {
        repo_list.add_repo(ipath)?;
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file)
            .expect("Unable to write");

        writeln!(file, "{}", ipath)?;

        Ok(())
    }
}
