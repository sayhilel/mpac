use clap::Parser;

pub struct Args {
    pub command: Option<Commands>,
    pub flag: usize,
}

pub enum Commands {
    Update,
    List,
}
