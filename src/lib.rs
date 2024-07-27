use std::path::PathBuf;

use clap::{Parser, Subcommand};

use anyhow::{anyhow, Context, Result};
use dialoguer::Confirm;

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
    pub directory: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    Init { directory: PathBuf },
}

pub fn init(directory: PathBuf) -> Result<()> {
    let full_path =
        std::fs::canonicalize(&directory).context("Trying to retrieve the canonical path")?;

    let res = Confirm::new()
        .with_prompt(format!(
            "Are you sure that you want to init spbased here: {:?}",
            full_path
        ))
        .interact()
        .context("tried to retrieve an answer from the user")?;

    if !res {
        return Err(anyhow!("User aborted init!"));
    }

    let spbased_dir = full_path.join(".spbased");

    // if an file with the path we want to use already exists, then exist
    if spbased_dir.is_file() {
        return Err(anyhow!("File with path {:?} already exists", spbased_dir));
    }

    let config = Config { directory };

    Ok(())
}
