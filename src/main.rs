//Temp
#![allow(unused)]

// Modules
mod cli;
mod git;
mod repo;
mod repolist;

use crate::cli::{Action, Cli};
use clap::Parser;
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config_path = Path::new("/home/croxymoc/Documents/Projects/mpac/target/debug/repos.txt");
    let mut repo_list = repolist::RepoList::new();
    repo_list.load(&config_path)?;

    match cli.action {
        Action::List => repo_list.list_repos(),
        Action::Update => repo_list.update_all(),
    }

    Ok(())
}
