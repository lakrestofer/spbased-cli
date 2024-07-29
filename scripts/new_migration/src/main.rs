use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};
use dialoguer::Input;
use time::{macros::format_description, OffsetDateTime};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    migration_directory: PathBuf,
}

fn main() -> Result<()> {
    let Cli {
        migration_directory,
    } = Cli::parse();

    let name: String = Input::new()
        .with_prompt("Name of migration")
        .interact_text()
        .unwrap();
    let name: String = name.split_whitespace().collect::<Vec<&str>>().join("_");

    let now = OffsetDateTime::now_utc();

    let now_str = now
        .format(format_description!(
            "[year][month][day][hour][minute][second]"
        ))
        .unwrap();

    let dir_path = migration_directory.join(format!("{now_str}_{name}"));
    let up_path = dir_path.join("up.sql");
    let down_path = dir_path.join("down.sql");

    std::fs::create_dir_all(dir_path)?;
    std::fs::File::create_new(up_path)?;
    std::fs::File::create_new(down_path)?;

    Ok(())
}
