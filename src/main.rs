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
        cli::Kommandoer::RutinefilValider { file_path } => {
            rutinefil_valider(file_path);
        },
        cli::Kommandoer::RutinefilVariabler {file_path} => {
            rutinefil_variabler(file_path)
        }
    }
}
