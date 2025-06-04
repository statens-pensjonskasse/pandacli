mod cli;
mod kommandoer;

use clap::Parser;
use cli::Cli;
use kommandoer::{rutinefil_validering::handle_verify_file};


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::RutinefilValider { file_path } => {
            handle_verify_file(file_path);
        }
    }
}
