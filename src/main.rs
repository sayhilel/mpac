//Temp
#![allow(unused)]

// Modules
mod cli;
mod git;
mod repolist;

use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("/home/croxymoc/Documents/Projects/manpac/target/debug/repos.txt");
    let mut repo_list = repolist::RepoList::new(&config_path)?;
    repo_list.list_repos();
    git::_pull(&repo_list.repos[0]);

    Ok(())
}
