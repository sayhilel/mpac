//Temp
#![allow(unused)]

// Modules
mod cli;
mod repo;
mod repos;

// Use
use clap::Parser;
use cli::Args;
use git2::Repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    println!("Command::{}, Flag::{}", args.command, args.flag);

    //Test Object
    let repos = repos::new().unwrap();

    repos.list_repos();

    Ok(())
}
