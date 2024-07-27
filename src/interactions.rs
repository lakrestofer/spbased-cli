pub mod filepath_completer;

use std::path::PathBuf;

use inquire::Confirm;

use crate::Config;

use self::filepath_completer::PathPromt;

pub fn promt_path(message: &'static str) -> Option<PathBuf> {
    PathPromt::new(message)
        .prompt_skippable()
        .unwrap_or(None)
        .map(PathBuf::from)
}
pub fn confirm(config: &Config) -> Option<bool> {
    let full_path = std::fs::canonicalize(&config.dir).ok().or(None)?;

    Confirm::new(&format!(
        "Are you sure that you want to init spbased at this location? {:?}",
        full_path
    ))
    .prompt_skippable()
    .unwrap_or(None)
}
