mod cli;
mod kommandoer;

use clap::Parser;
use cli::Cli;
use kommandoer::{rutinefil_validering::valider_rutinefil};


fn main() {
    let cli = Cli::parse();

    match &cli.kommando {
        cli::Kommandoer::RutinefilValider { file_path } => {
            valider_rutinefil(file_path);
        }
    }
}
