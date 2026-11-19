mod cli;
mod config;
mod kommandoer;
mod utils;

use clap::Parser;
use cli::Cli;
use std::process::ExitCode;

use crate::kommandoer::panda_murmur::panda_murmur;
use crate::kommandoer::rutinefil_variabler::rutinefil_variabler;
use kommandoer::diff_filer::diff_filer;
use kommandoer::header::header;
use kommandoer::operasjoner::operasjoner;
use kommandoer::rutinefil_validering::rutinefil_valider;
use kommandoer::summer_verdier::summer_verdier;
use kommandoer::velg_tilfeldig::velg_tilfeldig;
use uuid::Uuid;

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Hver kommando gir `true` ved suksess. Feil gir exit-kode 1 slik at
    // pcli kan brukes i skript og CI (f.eks. `pcli valider *.json || exit 1`).
    let suksess = match &cli.kommando {
        cli::Kommandoer::VelgTilfeldig { verdier } => velg_tilfeldig(verdier.clone()),

        cli::Kommandoer::RutinefilValider {
            file_paths,
            streng_metainfo,
        } => {
            // Validerer alle filene (ingen kortslutning), feiler hvis minst én feilet.
            let mut alle_gyldige = true;
            for path in file_paths {
                alle_gyldige &= rutinefil_valider(path, *streng_metainfo);
            }
            alle_gyldige
        }

        cli::Kommandoer::RutinefilVariabler { file_path } => rutinefil_variabler(file_path),

        cli::Kommandoer::CsvSummering {
            kolonne_nr,
            file_paths,
        } => match summer_verdier(*kolonne_nr, file_paths) {
            Ok(summering) => {
                for f in &summering.feil {
                    eprintln!("❌ {}", f);
                }

                if summering.antall_summert == 0 {
                    eprintln!("Ingen filer kunne summeres.");
                    return ExitCode::FAILURE;
                }

                let sum = utils::formater_tall(summering.sum);
                // Sortert for forutsigbar utskrift (HashMap er uordnet).
                let mut headere: Vec<_> = summering.headere.iter().collect();
                headere.sort_by(|a, b| a.0.cmp(b.0));
                let vis_header = |h: &str| {
                    if h.is_empty() {
                        "(ingen header)".to_string()
                    } else {
                        format!("'{}'", h)
                    }
                };

                if let [(header, _)] = headere.as_slice() {
                    println!(
                        "Summerte {} filer med header {}, sum: {}",
                        summering.antall_summert,
                        vis_header(header),
                        sum
                    );
                } else {
                    println!("Summerte {} filer, sum: {}", summering.antall_summert, sum);
                    println!(
                        "Fant {} forskjellige headere for kolonne {}:",
                        headere.len(),
                        kolonne_nr
                    );
                    for (header, filer) in headere {
                        let file_list = if filer.len() > 5 {
                            let første_fem: Vec<&str> =
                                filer.iter().take(5).map(AsRef::as_ref).collect();
                            format!("{} ... og {} flere", første_fem.join(", "), filer.len() - 5)
                        } else {
                            filer.join(", ")
                        };
                        println!("  - {}: {}", vis_header(header), file_list);
                    }
                }

                if !summering.feil.is_empty() {
                    eprintln!(
                        "❌ {} av {} filer kunne ikke summeres og er ikke med i summen.",
                        summering.feil.len(),
                        summering.antall_totalt()
                    );
                }
                summering.feil.is_empty()
            }
            Err(e) => {
                eprintln!("Feil under summering: {}", e);
                false
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
            false
        }

        cli::Kommandoer::Operasjoner { file_path, status } => {
            let result = operasjoner(file_path);

            if *status {
                let status_str = if result.is_ok() { "OK" } else { "FEILET" };
                println!("{}", status_str);
            } else {
                match &result {
                    Ok(value) => println!("{}", value),
                    Err(e) => eprintln!("Feil: {}", e),
                }
            }
            result.is_ok()
        }

        cli::Kommandoer::Header { file_path } => match header(file_path) {
            Ok(headere) => {
                println!("Fant {} headere:", headere.len());
                println!("{}", headere.join("\n"));
                true
            }
            Err(e) => {
                eprintln!("Feil: {}", e);
                false
            }
        },

        cli::Kommandoer::Filpartisjon { id } => {
            let result = panda_murmur(id);
            let unwrapped = result.unwrap();

            println!(
                "Plasseres i partisjon {}. Eksempel på filnavn: <filnavn>{}-{}.csv",
                unwrapped,
                unwrapped,
                Uuid::new_v4()
            );
            true
        }
    };

    if suksess {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
