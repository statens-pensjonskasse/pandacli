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
    #[command(about = "Validerer rutinefil ved å sjekke for definerte og brukte variabler")]
    RutinefilValider {
        file_path: String,
    },

    #[clap(name = "variabler")]
    #[command(about = "Finner og lister ut variabler i rutinefilen")]
    RutinefilVariabler {
        file_path: String,
    },
}