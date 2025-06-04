use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "pnd")]
#[command(about = "Ett kommandolinje-verktøy for bruk av premieleveranse", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub kommando: Kommandoer,
}

#[derive(Subcommand)]
pub enum Kommandoer {
    RutinefilValider {
        file_path: String,
    },
}