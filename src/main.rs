use clap::Parser;
use spbased_cli::{init, Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { dir, name } => init(dir, name),
    };
    // the the command succeed
}
