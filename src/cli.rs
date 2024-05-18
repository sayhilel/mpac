use clap::{Parser, Subcommand, ValueEnum};

pub struct Config {
    file: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            file: "/home/croxymoc/.config/mpac.conf".to_string(),
        }
    }

    pub fn path(&self) -> String {
        self.file.clone()
    }
}

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
    //    Add,
}
