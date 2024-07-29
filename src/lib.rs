use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: PathBuf },
}

pub mod commands {
    use crate::db::DB;
    use anyhow::{Context, Result};
    use dialoguer::Confirm;
    use normalize_path::NormalizePath;
    use resolve_path::PathResolveExt;
    use std::path::PathBuf;

    pub fn init(directory: PathBuf) -> Result<()> {
        let full_path: PathBuf = directory.try_resolve()?.into_owned().normalize();

        let res = Confirm::new()
            .with_prompt(format!(
                "Are you sure that you want to init spbased here: {:?}",
                full_path
            ))
            .interact()
            .context("tried to retrieve an answer from the user")?;

        if !res {
            println!("Goodbye!");
            return Ok(());
        }

        let spbased_dir = full_path.join(".spbased");

        // if an file with the path we want to use already exists, then exist
        if spbased_dir.exists() {
            let res = Confirm::new()
            .with_prompt(format!(
                "A directory called .spbased already exists at {:?}. Are you sure that you want to (re)init spased here?",
                full_path
            ))
            .interact()
            .context("tried to retrieve an answer from the user")?;

            if !res {
                return Ok(());
            }
        }

        // create the directory
        std::fs::create_dir_all(&spbased_dir)?;

        let db_path = spbased_dir.join("db.sqlite");
        DB::init(&db_path)?;

        Ok(())
    }
}

pub mod db {
    use anyhow::{Context, Result};
    use include_dir::{include_dir, Dir};
    use rusqlite::Connection;
    use rusqlite_migration::Migrations;
    use std::{cell::LazyCell, path::Path};

    pub struct DB;

    static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
    const MIGRATIONS: LazyCell<Migrations> =
        LazyCell::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

    impl DB {
        pub fn init(db_path: &Path) -> Result<()> {
            // if a file with the name db_path exist, we delete it
            if db_path.exists() {
                std::fs::remove_file(db_path)?;
            }

            // open and create a sqlite db
            let mut conn = Connection::open(db_path).context("trying to open connection")?;

            // run migrations on it
            MIGRATIONS
                .to_latest(&mut conn)
                .context("Trying to migrate sqlite schema")?;

            Ok(())
        }
    }
}
