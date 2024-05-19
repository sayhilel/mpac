use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "mpac")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub action: Action,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    List,
    Update,
    Add,
}
