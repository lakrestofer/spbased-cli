use std::{
    cell::{LazyCell, OnceCell},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};

use anyhow::{anyhow, Context, Result};
use dialoguer::Confirm;
use rusqlite::Connection;
use rusqlite_migration::Migrations;

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
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
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
        return Err(anyhow!(
            "File with path {:?} already exists. Please delete before trying again",
            spbased_dir
        ));
    }

    // create the directory
    std::fs::create_dir_all(spbased_dir)?;

    let config = Config { directory };

    Ok(())
}

pub struct DB {
    conn: Connection,
}

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
const MIGRATIONS: LazyCell<Migrations> =
    LazyCell::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

impl DB {
    pub fn create(directory: &Path) -> Result<()> {
        let db_path = directory.join("db.sqlite");

        let mut conn = Connection::open(db_path).context("trying to open connection")?;

        MIGRATIONS
            .to_latest(&mut conn)
            .context("Trying to migrate sqlite schema")?;

        Ok(())
    }
}
