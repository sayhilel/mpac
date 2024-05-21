use crate::repo::Repo;
use ::std::process::Command;
use anyhow::{anyhow, Result};
use colored::Colorize;

// Runs a "git pull <path>" command
// TO-DO
// Switch to a library like git2 for significanly better performance + cross platform-ness
// Implement differenciation between successful pull when an update is available versus when there
// isn't
pub async fn _pull(repo: &Repo) -> Result<()> {
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
        false => Err(anyhow!(": git {}", output.status)),
    }
}
