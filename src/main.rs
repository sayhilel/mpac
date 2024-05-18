//Temp
#![allow(unused)]

// Modules
mod cli;
mod git;
mod repo;
mod repolist;

use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("/home/croxymoc/Documents/Projects/mpac/target/debug/repos.txt");
    let mut repo_list = repolist::RepoList::new();
    repo_list.load(&config_path)?;

    git::_pull(&repo_list.repos[0]);
    git::_pull(&repo_list.repos[1]);
    Ok(())
}
