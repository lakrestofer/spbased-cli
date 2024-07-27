use clap::Parser;
use spbased_cli::{init, Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { dir } => init(dir),
    };
    // the the command succeed
}
