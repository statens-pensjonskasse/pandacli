mod cli;
mod config;
mod kommandoer;

use clap::Parser;
use cli::Cli;

use crate::kommandoer::rutinefil_variabler::rutinefil_variabler;
use kommandoer::rutinefil_validering::rutinefil_valider;
use kommandoer::summer_verdier::summer_verdier;

fn main() {
    let cli = Cli::parse();

    match &cli.kommando {
        cli::Kommandoer::RutinefilValider { file_paths } => {
            for path in file_paths {
                rutinefil_valider(path);
            }
        }

        cli::Kommandoer::RutinefilVariabler { file_path } => rutinefil_variabler(file_path),

        cli::Kommandoer::CsvSummering {
            kolonne_nr,
            file_paths,
        } => {
            let _ = summer_verdier(kolonne_nr, file_paths); //ignorerer resultatet midlertidig
        }
    }
}
