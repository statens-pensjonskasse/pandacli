mod cli;
mod kommandoer;
mod config;

use clap::Parser;
use cli::Cli;

use kommandoer::{rutinefil_validering::rutinefil_valider};
use crate::kommandoer::rutinefil_variabler::rutinefil_variabler;

fn main() {
    let cli = Cli::parse();

    match &cli.kommando {
        cli::Kommandoer::RutinefilValider { file_paths } => {
            for path in file_paths {
                rutinefil_valider(path);
            }
        },
        cli::Kommandoer::RutinefilVariabler {file_path} => {
            rutinefil_variabler(file_path)
        }
    }
}
