use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub command: String,
    pub flag: usize,
}
