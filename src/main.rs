//Temp
#![allow(unused)]

// Modules
mod cli;
mod repo;

// Use
use clap::Parser;
use cli::Args;
use git2::Repository;

use crate::repo::load_repos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    println!("Command::{}, Flag::{}", args.command, args.flag);

    //Test Object
    let repos = load_repos();

    println!("{}", repos.unwrap()[0].path);

    Ok(())
}
