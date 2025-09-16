use flate2::read::GzDecoder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use indicatif::{ProgressBar, ProgressStyle};

pub fn summer_verdier(
    kolonne_nr: usize,
    filstier: &[String] ,
) -> Result<(usize, f64, HashMap<String, Vec<String>>), String> {

    let pb = ProgressBar::new(filstier.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] {bar:30.green/black} {pos}/{len} ({percent}%)"
        )
            .unwrap()
            .progress_chars("█▇▆▅▄▃▂▁  ")
    );

    let pb_clone = pb.clone();
    let totals: Vec<(String, String, f64)> = filstier
        .par_iter()
        .map(|filsti| {
            let res = finn_sum_for_fil(kolonne_nr, filsti).ok().unwrap_or((
                String::new(),
                String::new(),
                0f64,
            ));
            pb_clone.inc(1);    // thread-safe increment
            res
        })
        .collect();

    let mut headere: HashMap<String, Vec<String>> = HashMap::new();
    let mut totalsum = 0.0;
    let antall_filer: usize = filstier.len();

    for (name, file, sum) in totals {
        totalsum += sum;
        headere.entry(name).or_insert_with(Vec::new).push(file);
    }

    pb.finish_with_message("All files processed");

    Ok((antall_filer, totalsum, headere))
}

fn finn_sum_for_fil(
    kolonne_nr: usize,
    filsti: &String,
) -> Result<(String, String, f64), Box<dyn Error>> {

    let mut headere: String = "".to_string();
    let filnavn: String = filsti
        .clone()
        .split("/")
        .last()
        .unwrap_or("ukjent")
        .to_string();

    let innhold = les_innhold_fil(filsti).ok().unwrap();

    let rader: Vec<&str> = innhold.lines().collect();
    if rader.is_empty() {
        return Ok((String::new(), String::new(), 0f64)); // Returnerer 0 for tom fil
    }

    // Sjekker om første element i fila IKKE er et tall.
    let har_header = rader
        .get(0)
        .and_then(|rad| rad.split(';').nth(kolonne_nr))
        .map_or(true, |verdi| verdi.trim().parse::<f64>().is_err());

    if har_header {
        if let Some(header_verdi) = rader.get(0).and_then(|rad| rad.split(';').nth(kolonne_nr)) {
            headere = header_verdi.trim().to_string();
        }
    }
    //todo: Legge til informasjon om filer som ikke har header

    let mut beløp_fil: f64 = 0.0;
    for rad in rader.iter().skip(if har_header { 1 } else { 0 }) {
        let kolonner: Vec<&str> = rad.split(';').collect();

        if kolonne_nr >= kolonner.len() {
            return Err(Box::from(format!(
                "Kolonne {} finnes ikke i filen '{}'.",
                kolonne_nr + 1,
                filsti
            )));
        }

        let kolonne_str = kolonner[kolonne_nr].trim();
        if kolonne_str.is_empty() {
            //Ingen verdi i kolonnen, hopp over
        } else if let Ok(verdi) = kolonne_str.parse::<f64>() {
            beløp_fil += verdi;
        } else {
            return Err(Box::from(format!(
                "Kunne ikke parse verdi '{}' som et tall i filen '{}'.",
                kolonner[kolonne_nr], filsti
            )));
        }
    }

    Ok((headere, filnavn, beløp_fil))
}

fn les_innhold_fil(filsti: &String) -> Result<String, Box<dyn Error>> {
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
        return Err(Box::from(format!(
            "Filen '{}' er ikke et støttet format.",
            filsti
        )));
    }

    Ok(innhold)
}
