use ::std::process::Command;
use std::process::Output;

use crate::repolist::RepoPath;

pub fn _pull(path: &RepoPath) {
    let loc = &path.path;
    println!("{}", loc);

    let output = Command::new("git")
        .arg("-C")
        .arg(loc)
        .arg("pull")
        .output()
        .expect("Err");

    println!("status: {}", output.status);
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
