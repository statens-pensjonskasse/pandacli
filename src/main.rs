mod cli;
mod config;
mod kommandoer;
mod utils;

use clap::Parser;
use cli::Cli;

use crate::kommandoer::rutinefil_variabler::rutinefil_variabler;
use kommandoer::diff_filer::diff_filer;
use kommandoer::header::header;
use kommandoer::rutinefil_validering::rutinefil_valider;
use kommandoer::summer_verdier::summer_verdier;
use kommandoer::velg_tilfeldig::velg_tilfeldig;
use kommandoer::operasjoner::operasjoner;
use uuid::Uuid;
use crate::kommandoer::panda_murmur::panda_murmur;

fn main() {
    let cli = Cli::parse();

    match &cli.kommando {

        cli::Kommandoer::VelgTilfeldig {verdier} => {
            velg_tilfeldig(verdier.clone());
        }

        cli::Kommandoer::RutinefilValider { file_paths } => {
            for path in file_paths {
                rutinefil_valider(path);
            }
        }

        cli::Kommandoer::RutinefilVariabler { file_path } => rutinefil_variabler(file_path),

        cli::Kommandoer::CsvSummering {
            kolonne_nr,
            file_paths,
        } => match summer_verdier(*kolonne_nr, file_paths) {
            Ok((antall_filer, sum, headere)) => {

                if headere.len() > 1 {
                    println!(
                        "Summerer beløp fra {} filer, sum: {}",
                        antall_filer,
                        utils::formater_tall(sum)
                    );
                    println!(
                        "Fant {} forskjellige headere for kolonne {}. Summert kolonner fra {} filer, sum: {}",
                        headere.len(),
                        kolonne_nr + 1,
                        antall_filer,
                        utils::formater_tall(sum)
                    );

                    println!("Headere og deres respektive filer:");
                    for (header, filer) in &headere {
                        let file_list = if filer.len() > 5 {
                            let første_fem: Vec<&str> =
                                filer.iter().take(5).map(AsRef::as_ref).collect();
                            format!("{} ... og {} flere", første_fem.join(", "), filer.len() - 5)
                        } else {
                            filer.join(", ")
                        };
                        println!("  - '{}': {}", header, file_list);
                    }
                } else {
                    println!(
                        "Summerer fra {} filer med header '{}', sum: {}",
                        antall_filer,
                        headere.keys().next().unwrap_or(&"Ingen header".to_string()),
                        utils::formater_tall(sum)
                    );
                }
            }
            Err(e) => {
                eprintln!("Feil under summering: {}", e)
            }
        },
        cli::Kommandoer::Diff {
            venstre,
            høyre,
            ignorer,
        } => {
            let _ = diff_filer(venstre.to_vec(), høyre.to_vec(), ignorer.to_vec()); //ignorerer resultatet midlertidig
            eprintln!("Diff kommando er ikke implementert ennå.");
            eprintln!("Bruk 'pcli --help' for mer informasjon om tilgjengelige kommandoer.");
        }

        cli::Kommandoer::Operasjoner { file_path, status } => {
            let result = operasjoner(file_path);

            if *status == true {
                let status_str = if result.is_ok() { "OK" } else { "FEILET" };
                eprintln!("{}", status_str);
            } else {
                match result {
                    Ok(value) => eprintln!("{}", value),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }

        cli::Kommandoer::Header { file_path } => {
            let result = header(file_path);

            let unwrapped = result.unwrap();

            eprintln!("Fant {} headere:", unwrapped.len());
            eprintln!("{}", unwrapped.join("\n"));
        },

        cli::Kommandoer::Filpartisjon {
            id
        } => {
            let result = panda_murmur(id);
            let unwrapped = result.unwrap();

            eprintln!("Plasseres i partisjon {}. Eksempel på filnavn: <filnavn>{}-{}.csv",
                      unwrapped,
                      unwrapped,
                      Uuid::new_v4());
        }
    }
}
