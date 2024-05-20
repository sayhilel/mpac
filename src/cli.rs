use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "mpac")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand)]
pub enum Action {
    List,
    Update,
    Add {
        #[arg(long)]
        repo: String,
    },
}
