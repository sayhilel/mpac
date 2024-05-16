//Temp
#![allow(unused)]

// Modules
mod cli;
mod git;
mod repos;

// Use
use clap::Parser;
use cli::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    println!("Command::{}, Flag::{}", args.command, args.flag);

    //Test Object
    let repos = repos::new().unwrap();

    repos.list_repos();

    Ok(())
}
