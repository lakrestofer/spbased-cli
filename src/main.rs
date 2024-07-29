use anyhow::Result;
use clap::Parser;
use spbased_cli::{Cli, Commands};

use spbased_cli::commands::*;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { directory } => init(directory),
    }?;
    // the the command succeed
    Ok(())
}
