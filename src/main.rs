//Temp
#![allow(unused)]

// Modules
mod cli;
mod file;
mod git;
mod repo;

use crate::cli::{Action, Cli};
use crate::file::Config;
use anyhow::Error;
use clap::Parser;
use std::path::Path;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let config = Config::default();
    let config_path = Path::new(&config.file.trim());
    let mut repo_list = repo::RepoList::new();
    config.load(&mut repo_list)?;

    match cli.action {
        Action::List => repo_list.list(),
        Action::Update => repo_list.update_all(),
        Action::Add => config.add_to_file(
            &mut repo_list,
            &"/home/croxymoc/Documents/Projects/Portfolio/Sayhilel/".to_string(),
        )?,
    }

    Ok(())
}
