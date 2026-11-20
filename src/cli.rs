use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "panda-cli",
    version = env!("CARGO_PKG_VERSION"),
    about = "Kommandolinje-verktøy for bruk av premieleveranse. Skriv 'pcli <kommando> --help' for mer informasjon om en spesifikk kommando.",
    long_about = "Dette verktøyet gir enkle kommandoer for å validere rutinefiler, \
                      hente ut variabler fra rutinefiler, summere verdier i CSV-filer, \
                      følge med på kjøringer og finne filpartisjon for et medlem.",
    after_help = "EXIT-KODER:\n  \
                  0  Kommandoen lyktes\n  \
                  1  Kommandoen feilet (f.eks. valideringsfeil, fil som ikke kunne leses eller summeres)\n  \
                  2  Ugyldige argumenter"
)]
pub struct Cli {
    #[command(subcommand)]
    pub kommando: Kommandoer,
}

#[derive(Subcommand)]
pub enum Kommandoer {
    #[clap(name = "rnd")]
    #[command(
        about = "Velger en tilfeldig verdi fra input.",
        long_about = "Tar en liste med verdier som input, og velger en tilfeldig verdi.",
        after_help = "EKSEMPLER:\n  \
                      pcli rnd a b c d e f ..."
    )]
    VelgTilfeldig {
        #[arg(required = true)]
        verdier: Vec<String>,
    },

    #[clap(name = "valider")]
    #[command(
        about = "Validerer en eller flere rutinefiler mot det panda-orkestrering støtter.",
        long_about = "Validerer en eller flere rutinefiler mot en katalog over hva panda-orkestrering støtter: \
                      variabler (manglende, uutfylte og ubrukte), kjente handlinger og #{...}-funksjoner, struktur, \
                      listevariabler, versjonsformat og grunnlagsdataMappe for batcher. \
                      Rutinefilene må være i JSON-format og inneholde et 'variabler'-felt.",
        after_help = "EKSEMPLER:\n  \
                      pcli valider fil1.json fil2.json ...\n  \
                      pcli valider *.json\n  \
                      pcli valider --streng-metainfo mal.json\n\n\
                      EXIT-KODE:\n  \
                      1 hvis minst én fil har feil (❌) eller ikke kan leses. Advarsler om uutfylte (❓)\n  \
                      eller ubrukte (⚠️) variabler gir ikke feil, siden maler har uutfylte plassholdere.\n"
    )]
    RutinefilValider {
        #[arg(required = true)]
        file_paths: Vec<String>,
        #[clap(
            long = "streng-metainfo",
            short = 'm',
            help = "Slå på streng sjekk av 'metainfo.mal' (at filnavnet matcher malen). Av som standard fordi filer ofte blir omdøpt før kjøring."
        )]
        streng_metainfo: bool,
    },

    #[clap(name = "variabler")]
    #[command(
        about = "Finner og lister ut variabler i rutinefilen",
        long_about = "Henter ut alle variabler som er definert i en rutinefil. Rutinefilen må være i JSON-format og inneholde et 'variabler'-felt.",
        after_help = "EKSEMPLER:\n  \
                      pcli variabler fil.json"
    )]
    RutinefilVariabler { file_path: String },

    #[clap(name = "summer")]
    #[command(
        about = "Summerer beløp i en gitt kolonne for alle rader i csv-fil(ene), støtter også gz komprimerte filer.",
        long_about = "Summerer verdier i en spesifisert kolonne (0-indeksert) for en eller flere CSV-filer, tomme felt blir håndtert som om de var 0. \
                      Filene kan være vanlige CSV-filer (.csv) eller GZip-komprimerte CSV-filer (.csv.gz). \
                      Kommandoen varsler fra om det er forskjellige headere mellom filer. \
                      Filer som ikke kan summeres rapporteres med linjenummer, er ikke med i summen, og gir exit-kode 1. \
                      NB! Er du usikker på hvilken kolonne du skal bruke, kan du bruke kommandoen 'pcli header <filnavn>' for å liste opp headere med indekser.",
        after_help = "EKSEMPLER:\n  \
                      pcli summer 0 fil1.csv fil2.csv.gz\n  \
                      pcli summer 0 *.csv\n  \
                      pcli summer 2 data/min_fil.csv"
    )]
    CsvSummering {
        kolonne_nr: usize,
        #[arg(required = true)]
        file_paths: Vec<String>,
    },

    #[clap(name = "operasjoner")]
    #[command(
        about = "Finner hvor langt rutinefilen har kommet ved å sjekke antall handlinger som er gjennomført.",
        long_about = "Printer hvor langt en kjøring har kommet ved å sjekke antall handlinger som er gjennomført.",
        after_help = "EKSEMPLER:\n  \
                  pcli operasjoner operasjoner.log\n  \
                  pcli operasjoner operasjoner.log --status\n  \
                  pcli operasjoner operasjoner.log -s"
    )]
    Operasjoner {
        file_path: String,
        #[clap(
            short,
            long,
            help = "Vis kun status for kjøringen. OK hvis kjøringen er ferdig eller kjører, FEILET hvis det oppstod en feil."
        )]
        status: bool,
    },

    #[clap(name = "header")]
    #[command(
        about = "Printer ut headere til en csv fil, med index for hvert felt.",
        long_about = "Printer ut headere til en csv fil, med index for hvert felt.",
        after_help = "EKSEMPLER:\n  \
                      pcli header fil.csv"
    )]
    Header { file_path: String },
    #[clap(name = "filpartisjon")]
    #[command(
        about = "Printer ut hvilken partisjon et fødselsnummer eller en ID tilhører.",
        long_about = "Ved å bruke samme murmur hashing som tidsserie-lib bruker, kan man finne ut hvilke filpartisjon ett medlem havner i.\
        Dette kan brukes til å legge medlemmer i en filpartisjon manuelt, eller ved å opprette smoke tester i cucumber for prosjekter som har trenger definert filpartisjon.",
        after_help = "EKSEMPLER:\n  \
                      pcli filpartisjon D9F47A69-E85E-42A0-9686-151835F38ACB"
    )]
    Filpartisjon { id: String },
}
