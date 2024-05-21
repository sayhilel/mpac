mod cli;
mod file;
mod git;
mod repo;

use crate::cli::{Action, Cli};
use crate::file::Config;
use anyhow::Error;
use clap::Parser;
use colored::Colorize;

#[tokio::main]
// TO-DO
// Add more features such as individual updates
// Maybe make this more of rust wrapper for working on multiple repos?
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let config = Config::default();
    let mut repo_list = repo::RepoList::new();
    let _ = config.load(&mut repo_list).await;

    match &cli.action {
        Some(Action::List) => repo_list.list(),
        Some(Action::Update) => repo_list.update_all().await,
        Some(Action::Add { repo }) => config.add_to_file(&mut repo_list, &repo).await?,
        Some(Action::Rm { index }) => config.remove_from_file(&mut repo_list, *index).await?,
        None => println!("{}", "No action specified, rerun with \"--help\"".yellow()),
    }

    Ok(())
}
