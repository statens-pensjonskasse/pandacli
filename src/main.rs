mod cli;
mod config;
mod kommandoer;

use clap::Parser;
use cli::Cli;

use crate::kommandoer::rutinefil_variabler::rutinefil_variabler;
use kommandoer::rutinefil_validering::rutinefil_valider;

fn main() {
    let cli = Cli::parse();

    match &cli.kommando {
        cli::Kommandoer::RutinefilValider { file_paths } => {
            for path in file_paths {
                rutinefil_valider(path);
            }
        }

        cli::Kommandoer::RutinefilVariabler { file_path } => rutinefil_variabler(file_path),

        #[warn(unused_variables)]
        cli::Kommandoer::CsvSummering {
            kolonne_nr: _,
            file_paths,
        } => {
            for _path in file_paths {
                eprintln!("Ikke implementert: Summering av CSV-filer.");
            }
        }
    }
}
