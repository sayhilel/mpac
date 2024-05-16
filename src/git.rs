use git2::Repository;
use std::path::Path;

// git2 function implementations
pub fn fetch_(repo: &mut Repository, flag: usize) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn pull_(repo: &mut Repository) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
