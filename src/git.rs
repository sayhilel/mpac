use crate::repo::Repo;
use ::std::process::Command;
use anyhow::{anyhow, Result};
use colored::Colorize;

pub async fn _pull(repo: &Repo) -> Result<()> {
    // No hashmap / easy lookup for path
    let loc = &repo.path.to_str().unwrap();

    let output = Command::new("git")
        .arg("-C")
        .arg(loc)
        .arg("pull")
        .output()
        .expect("Err");

    match output.status.success() {
        true => {
            println!("{}", "Updating...".yellow());
            return Ok(());
        }
        false => Err(anyhow!("git error code:{}", output.status)),
    }
}
