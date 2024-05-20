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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let config = Config::default();
    let config_path = Path::new(&config.file.trim());
    let mut repo_list = repo::RepoList::new();
    let config_load = config.load(&mut repo_list).await;

    match &cli.action {
        Some(Action::List) => repo_list.list(),
        Some(Action::Update) => repo_list.update_all().await,
        Some(Action::Add { repo }) => config.add_to_file(&mut repo_list, &repo).await?,
        None => println!("No action specified, rerun with \"--help\" "),
    }

    Ok(())
}

