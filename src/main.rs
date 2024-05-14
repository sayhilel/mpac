//Temp
#![allow(unused)]

// Modules
mod cli;
mod repo;

// Use
use clap::Parser;
use cli::Args;
use repo::Repo;

fn main() {
    let args = cli::Args::parse();
    println!("Command::{}, Flag::{}", args.command, args.flag);

    //Test Object
    let path = "test".to_string();
    let time = 5;
    let test_repo = Repo { path, time };
}
