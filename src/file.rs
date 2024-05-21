use crate::repo::RepoList;
use anyhow::{anyhow, Result};
use colored::Colorize;
use dirs::config_dir;
use std::io::prelude::*;

use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

pub struct Config {
    pub file: String,
}

// TO-DO
// Custom Config File & Validation For It
impl Config {
    pub fn default() -> Self {
        let mut config_ = config_dir()
            .expect(
                &"Unable to locate config directory. Make sure \"/home/<user>/.config\" exists."
                    .red(),
            )
            .to_str()
            .expect(&"Config path might have unresolved characters".red())
            .to_string();
        config_ += "/mpac.conf";
        Self { file: config_ }
    }

    // Creates a Repo Struct for all values in config file
    pub async fn load(&self, repo_list: &mut RepoList) -> Result<()> {
        let repo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file)
            .expect(
                &"Unable to create config file. Make sure you own \"/home/<user>/.config\".".red(),
            );

        let reader = BufReader::new(repo_file);

        for lines in reader.lines() {
            if let Ok(path) = lines {
                repo_list.add_repo(&path.trim()).await?
            } else {
                println!(
                    "{}{}",
                    "Unable to read config file. Make sure you own ".red(),
                    &self.file.red()
                )
            }
        }

        Ok(())
    }

    // Adds Repo to config file
    pub async fn add_to_file(&self, repo_list: &mut RepoList, ipath: &String) -> Result<()> {
        let check = &ipath.clone();

        if repo_list.lookup.contains_key(check.trim()) {
            return Err(anyhow!("Repo already exist in config file"));
        }

        repo_list.add_repo(ipath).await?;
        let mut file = OpenOptions::new().append(true).open(&self.file).expect(
            &"Unable to write to config file. Make sure you own \"/home/<user>/.config\".".red(),
        );

        writeln!(file, "{}", ipath)?;

        Ok(())
    }

    // Removes a Repo from file given it's index in "mpac list"
    pub async fn remove_from_file(&self, repo_list: &mut RepoList, index: usize) -> Result<()> {
        if repo_list.repos.len() < index {
            return Err(anyhow!("{} {index} ", "Invalid index.".red()));
        }

        repo_list.repos.remove(index - 1);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file)
            .expect(&"Unable to rewrite to file, data might be corrupted".red());

        repo_list.repos.iter().for_each(|x| {
            writeln!(file, "{}", &x.path.display())
                .expect(&"Unable to rewrite to file, data might be corrupted".red())
        });

        Ok(())
    }
}
