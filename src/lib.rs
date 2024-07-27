use std::path::PathBuf;
pub mod interactions;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

pub struct Config {
    pub dir: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    Init { dir: Option<PathBuf> },
}

pub fn init(dir: Option<PathBuf>) {
    // retrieve or ask for path and name
    let dir =
        dir.or_else(|| interactions::promt_path("Path to direction where spbased will init: "));
    if dir.is_none() {
        return;
    };
    let dir = dir.unwrap();
    // let name = name.unwrap_or_else(|| interactions::promt_name());
    let config = Config { dir };

    // confirm choice
    if !interactions::confirm(&config).unwrap_or(false) {
        return;
    };
}
