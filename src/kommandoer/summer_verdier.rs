use flate2::read::GzDecoder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use indicatif::{ProgressBar, ProgressStyle};

pub fn summer_verdier(
    kolonne_nr: usize,
    filstier: &[String],
) -> Result<(usize, f64, HashMap<String, Vec<String>>), String> {
    let antall_filer = filstier.len();

    let pb = ProgressBar::new(antall_filer as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] {bar:30.green/black} {pos}/{len} ({percent}%)"
        )
            .unwrap()
            .progress_chars("█▇▆▅▄▃▂▁  "),
    );

    // Begrenser parallellisme for å unngå for mange samtidige fil-I/O-operasjoner ved store datamengder.
    // Hver tråd holder kun én BufReader (~8 KB) og én linje av gangen, minnebruk er O(tråder), ikke O(filer).
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get().min(8))
        .unwrap_or(4)
        .max(1);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(|e| format!("Klarte ikke opprette trådpool: {}", e))?;

    log_minnebruk("start summering");

    let pb_clone = pb.clone();
    let results: Vec<Result<(String, String, f64), String>> = pool.install(|| {
        filstier
            .par_iter()
            .map(|filsti| {
                let res = finn_sum_for_fil(kolonne_nr, filsti)
                    .map_err(|e| format!("{}: {}", filsti, e));
                pb_clone.inc(1);
                log_minnebruk(filsti);
                res
            })
            .collect()
    });
    drop(pool);

    let mut headere: HashMap<String, Vec<String>> = HashMap::with_capacity(8);
    let mut totalsum = 0.0f64;
    let mut feil: Vec<String> = Vec::new();

    for result in results {
        match result {
            Ok((name, file, sum)) => {
                totalsum += sum;
                headere.entry(name).or_default().push(file);
            }
            Err(e) => feil.push(e),
        }
    }

    if !feil.is_empty() {
        for f in &feil {
            eprintln!("Advarsel: {}", f);
        }
    }

    pb.finish_with_message("All files processed");
    log_minnebruk("ferdig summering");

    Ok((antall_filer, totalsum, headere))
}

fn finn_sum_for_fil(
    kolonne_nr: usize,
    filsti: &str,
) -> Result<(String, String, f64), Box<dyn Error>> {
    let filnavn = filsti.split('/').last().unwrap_or("ukjent").to_string();

    let reader = aapne_fil_som_leser(filsti)?;
    let mut linjer = reader.lines();

    // Leser kun første linje for å detektere header — ingen full fil-allokering.
    let første_linje = match linjer.next() {
        Some(Ok(l)) => l,
        Some(Err(e)) => return Err(Box::from(format!("Lese-feil i '{}': {}", filsti, e))),
        None => return Ok((String::new(), filnavn, 0.0)),
    };

    // Sjekker om første element i fila IKKE er et tall.
    let har_header = første_linje
        .split(';')
        .nth(kolonne_nr)
        .map_or(true, |v| v.trim().parse::<f64>().is_err());

    let header_str = if har_header {
        første_linje
            .split(';')
            .nth(kolonne_nr)
            .map(|v| v.trim().to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };
    //todo: Legge til informasjon om filer som ikke har header

    let mut beløp_fil = 0.0f64;

    // Hvis ingen header, er første linje også data.
    if !har_header {
        summer_rad(&første_linje, kolonne_nr, filsti, &mut beløp_fil)?;
    }
    // første_linje er ikke lenger nødvendig; slipp den før resten streames.
    drop(første_linje);

    // Strøm linje for linje: kun én String-allokering av gangen, slippes etter hver iterasjon.
    for linje_result in linjer {
        let linje = linje_result.map_err(|e| format!("Lese-feil i '{}': {}", filsti, e))?;
        summer_rad(&linje, kolonne_nr, filsti, &mut beløp_fil)?;
    }

    Ok((header_str, filnavn, beløp_fil))
}

/// Akkumulerer én semikolon-separert rad inn i `sum`. Bruker `nth()` på
/// split-iteratoren for å unngå å allokere en Vec av kolonnesegmenter.
fn summer_rad(
    linje: &str,
    kolonne_nr: usize,
    filsti: &str,
    sum: &mut f64,
) -> Result<(), Box<dyn Error>> {
    let kolonne_str = match linje.split(';').nth(kolonne_nr) {
        Some(v) => v.trim(),
        None => {
            return Err(Box::from(format!(
                "Kolonne {} finnes ikke i filen '{}'.",
                kolonne_nr + 1,
                filsti
            )))
        }
    };
    if kolonne_str.is_empty() {
        return Ok(());
    }
    match kolonne_str.parse::<f64>() {
        Ok(v) => {
            *sum += v;
            Ok(())
        }
        Err(_) => Err(Box::from(format!(
            "Kunne ikke parse verdi '{}' som et tall i filen '{}'.",
            kolonne_str, filsti
        ))),
    }
}

/// Returnerer en bufret leser for .csv og .gz filer.
/// BufReader gir ~8 KB I/O-buffer; GzDecoder dekomprimerer underveis.
/// Ingen av dem leser hele filen inn i minnet.
fn aapne_fil_som_leser(filsti: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    if filsti.ends_with(".gz") {
        let fil = File::open(filsti)
            .map_err(|e| format!("Klarte ikke åpne filen '{}': {}", filsti, e))?;
        Ok(Box::new(BufReader::new(GzDecoder::new(fil))))
    } else if filsti.ends_with(".csv") {
        let fil = File::open(filsti)
            .map_err(|e| format!("Klarte ikke åpne filen '{}': {}", filsti, e))?;
        Ok(Box::new(BufReader::new(fil)))
    } else {
        Err(Box::from(format!(
            "Filen '{}' er ikke et støttet format.",
            filsti
        )))
    }
}

        /// Logger resident set size (RSS) til stderr når PCLI_MEM_LOG=1 er satt.
        /// På Linux leses fra /proc/self/status. Gjør ingenting på andre plattformer.
        fn log_minnebruk(kontekst: &str) {
            if std::env::var_os("PCLI_MEM_LOG").is_none() {
                return;
            }
            #[cfg(target_os = "linux")]
            {
                if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
                    for linje in status.lines() {
                        if linje.starts_with("VmRSS:") {
                            if let Some(kb_str) = linje.split_whitespace().nth(1) {
                                if let Ok(kb) = kb_str.parse::<u64>() {
                                    eprintln!("[mem] RSS: {} MB | {}", kb / 1024, kontekst);
                                }
                            }
                            break;
                        }
                    }
                }
            }
            #[cfg(not(target_os = "linux"))]
            let _ = kontekst;
}
