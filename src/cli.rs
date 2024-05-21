use clap::{Parser, Subcommand};

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
        #[arg(short, long)]
        repo: String,
    },
    Rm {
        #[arg(short, long)]
        index: usize,
    },
}
