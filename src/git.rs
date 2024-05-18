use ::std::process::Command;
use std::process::ExitStatus;

use crate::repo::Repo;

pub fn _pull(repo: &Repo) {
    // No hashmap / easy lookup for path
    let loc = &repo.path.to_str().unwrap();

    let output = Command::new("git")
        .arg("-C")
        .arg(loc)
        .arg("pull")
        .output()
        .expect("Err");

    match output.status.success() {
        true => println!("{} was succesfully updated", &repo.name),
        false => {
            println!("Couldn't update {}", &repo.name);
            println!("Status Code {}", &output.status)
        }
    }
}
