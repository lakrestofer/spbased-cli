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
    pub name: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        dir: Option<PathBuf>,
        name: Option<String>,
    },
}

pub fn init(dir: Option<PathBuf>, name: Option<String>) {
    // retrieve or ask for path and name
    let dir = dir.or_else(|| interactions::promt_path());
    let name = name.or_else(|| interactions::promt_name());
    if dir.is_none() || name.is_none() {
        return;
    };
    let dir = dir.unwrap();
    let name = name.unwrap();
    // let name = name.unwrap_or_else(|| interactions::promt_name());
    let config = Config {
        dir,
        name: "something".into(),
    };

    // confirm choice
    if !interactions::confirm(&config) {
        return;
    };
}
