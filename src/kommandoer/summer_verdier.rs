use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn summer_verdier(
    kolonne_nr: usize,
    filstier: &[String],
) -> Result<(usize, f64, HashMap<String, Vec<String>>), String> {
    let mut total_sum: f64 = 0.0;
    let mut antall_filer: usize = 0;
    let mut headere: HashMap<String, Vec<String>> = HashMap::new();

    for filsti in filstier {
        antall_filer += 1;

        let mut innhold = String::new();
        if filsti.ends_with(".gz") {
            let tar_gz = File::open(filsti)
                .map_err(|e| format!("Klarte ikke åpne filen '{}': {}", filsti, e))?;

            let mut gz_decoder = GzDecoder::new(tar_gz);
            gz_decoder.read_to_string(&mut innhold).map_err(|e| {
                format!(
                    "Klarte ikke lese innholdet fra gz-filen '{}': {}",
                    filsti, e
                )
            })?;
        } else if filsti.ends_with(".csv") {
            innhold = std::fs::read_to_string(filsti)
                .map_err(|e| format!("Klarte ikke lese filen '{}': {}", filsti, e))?;
        } else {
            return Err(format!("Filen '{}' er ikke et støttet format.", filsti));
        }

        let rader: Vec<&str> = innhold.lines().collect();
        if rader.is_empty() {
            continue; // Hopp over tomme filer
        }

        // Sjekker om første element i fila IKKE er et tall.
        let har_header = rader
            .get(0)
            .and_then(|rad| rad.split(';').nth(kolonne_nr))
            .map_or(true, |verdi| verdi.trim().parse::<f64>().is_err());

        if har_header {
            if let Some(header_verdi) = rader.get(0).and_then(|rad| rad.split(';').nth(kolonne_nr))
            {
                headere
                    .entry(header_verdi.trim().to_string())
                    .or_default()
                    .push(
                        filsti
                            .clone()
                            .split("/")
                            .last()
                            .unwrap_or("ukjent")
                            .to_string(),
                    );
            }
        }
        //todo: Legge til informasjon om filer som ikke har header

        for rad in rader.iter().skip(if har_header { 1 } else { 0 }) {
            let kolonner: Vec<&str> = rad.split(';').collect();

            if kolonne_nr >= kolonner.len() {
                return Err(format!(
                    "Kolonne {} finnes ikke i filen '{}'.",
                    kolonne_nr + 1,
                    filsti
                ));
            }

            let kolonne_str = kolonner[kolonne_nr].trim();
            if kolonne_str.is_empty() {
                //Ingen verdi i kolonnen, hopp over
            } else if let Ok(verdi) = kolonne_str.parse::<f64>() {
                total_sum += verdi;
            } else {
                return Err(format!(
                    "Kunne ikke parse verdi '{}' som et tall i filen '{}'.",
                    kolonner[kolonne_nr], filsti
                ));
            }
        }
    }
    Ok((antall_filer, total_sum, headere))
}
