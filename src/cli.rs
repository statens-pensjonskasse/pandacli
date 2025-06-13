use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command( 
    name = "panda-cli",
    version = env!("CARGO_PKG_VERSION"),
    about = "Kommandolinje-verktøy for bruk av premieleveranse",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub kommando: Kommandoer,
}

#[derive(Subcommand)]
pub enum Kommandoer {
    #[clap(name = "valider")]
    #[command(about = "Validerer rutinefil ved å sjekke for definerte og brukte variabler. Kan validere en eller flere rutinefiler.")]
    RutinefilValider {
        file_paths: Vec<String>,
    },

    #[clap(name = "variabler")]
    #[command(about = "Finner og lister ut variabler i rutinefilen")]
    RutinefilVariabler {
        file_path: String,
    },

    #[clap(name = "summer")]
    #[command(about = "*IKKE IMPLEMENTERT* Summerer beløp i en gitt kolonne for alle rader i csv-fil(ene), støtter også gz komprimerte filer.")]
    CsvSummering {
        kolonne_nr: i8,
        file_paths: Vec<String>,
    },
}