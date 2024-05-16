//Temp
#![allow(unused)]

// Modules
mod cli;
mod repo;

// Use
use clap::Parser;
use cli::Args;
use git2::Repository;

use crate::repo::Repos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    println!("Command::{}, Flag::{}", args.command, args.flag);

    //Test Object
    let repos = repo::new().unwrap();

    repos.list_repos();

    Ok(())
}
