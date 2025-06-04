use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "pnd")]
#[command(about = "Ett kommandolinje-verktøy for bruk av premieleveranse", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    RutinefilValider {
        file_path: String,
    },
}