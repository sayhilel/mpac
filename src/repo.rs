use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    io::{self, prelude, BufRead, BufReader, Error},
};

// CHANGE-ME
// Keep track of a repo's path and last updated time
pub struct Repo {
    pub path: String,
    //pub time: usize,
}
