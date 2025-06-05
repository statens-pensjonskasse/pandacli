mod cli;
mod kommandoer;

use clap::Parser;
use cli::Cli;
use kommandoer::{rutinefil_validering::rutinefil_valider};


fn main() {
    let cli = Cli::parse();

    match &cli.kommando {
        cli::Kommandoer::RutinefilValider { file_path } => {
            rutinefil_valider(file_path);
        }
    }
}
