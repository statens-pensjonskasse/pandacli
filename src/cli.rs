use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "panda-cli",
    version = env!("CARGO_PKG_VERSION"),
    about = "Kommandolinje-verktøy for bruk av premieleveranse. Skriv 'pnd <kommando> --help' for mer informasjon om en spesifikk kommando.",
    long_about = "Dette verktøyet gir enkle kommandoer for å validere rutinefiler, \
                      hente ut variabler fra rutinefiler og summere verdier i CSV-filer."
)]
pub struct Cli {
    #[command(subcommand)]
    pub kommando: Kommandoer,
}

#[derive(Subcommand)]
pub enum Kommandoer {
    #[clap(name = "valider")]
    #[command(
        about = "Validerer rutinefil ved å sjekke for definerte og brukte variabler. Kan validere en eller flere rutinefiler.",
        long_about = "Denne kommandoen validerer en eller flere rutinefiler ved å sjekke at alle variabler som er definert i filen også er brukt. \
                      Rutinefilene må være i JSON-format og inneholde et 'variabler'-felt.",
        after_help = "EKSEMPLER:\n  \
                      pnd valider fil1.json fil2.json ...\n  \
                      pnd valider *.json\n"
    )]
    RutinefilValider { file_paths: Vec<String> },

    #[clap(name = "variabler")]
    #[command(
        about = "Finner og lister ut variabler i rutinefilen",
        long_about = "Henter ut alle variabler som er definert i en rutinefil.  Rutinefilen må være i JSON-format og inneholde et 'variabler'-felt.",
        after_help = "EKSEMPLER:\n  \
                      pnd variabler fil.json"
    )]
    RutinefilVariabler { file_path: String },

    #[clap(name = "summer")]
    #[command(
        about = "Summerer beløp i en gitt kolonne for alle rader i csv-fil(ene), støtter også gz komprimerte filer.",
        long_about = "Summerer verdier i en spesifisert kolonne (0-indeksert) for en eller flere CSV-filer, tomme felt blir håndtert som om de var 0. \
                      Filene kan være vanlige CSV-filer (.csv) eller GZip-komprimerte CSV-filer (.csv.gz).\
                      Kommandoen varsler fra om det er forskjellige headere mellom filer.",
        after_help = "EKSEMPLER:\n  \
                      pnd summer 0 fil1.csv fil2.csv.gz\n  \
                      pnd summer 0 *.csv\n  \
                      pnd summer 2 data/min_fil.csv"
    )]
    CsvSummering {
        kolonne_nr: usize,
        file_paths: Vec<String>,
    },

    #[clap(name = "diff")]
    #[command(
        about = "*Ikke implementert* Sammenlikner to sett med filer for å finne forskjeller.",
        long_about = "Denne kommandoen sammenligner to sett med filer og viser forskjeller mellom dem. \
                          Den kan brukes til å finne endringer i innholdet i filene.",
        after_help = "EKSEMPLER:\n  \
                          pnd diff --venstre fil1.txt fil2.txt --høyre fil3.txt fil4.txt\n  \
                          pnd diff --venstre original/*.txt --høyre nye/*.bak"
    )]
    Diff {
        #[clap(long, value_name = "VENSTRE_FILER", required = true, num_args = 1.., help = "Første sett med filer som skal sammenlignes")]
        venstre: Vec<String>,

        #[clap(long, value_name = "HØYRE_FILER", required = true, num_args = 1.., help = "Andre sett med filer som skal sammenlignes")]
        høyre: Vec<String>,

        #[clap(long, value_name = "ignorerte_kolonner", num_args = 1.., help = "Kolonner i CSV-filer som skal ignoreres under sammenligning")]
        ignorer: Vec<usize>,
    }
}
