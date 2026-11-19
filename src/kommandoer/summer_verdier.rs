use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Summering {
    /// Antall filer som ble summert uten feil (er med i `sum`).
    pub antall_summert: usize,
    pub sum: f64,
    /// Header for kolonnen -> filene som har den headeren. Filer uten header
    /// ligger under tom streng.
    pub headere: HashMap<String, Vec<String>>,
    /// Feilmeldinger for filer som ikke kunne summeres (er ikke med i `sum`).
    pub feil: Vec<String>,
}

impl Summering {
    /// Antall filer totalt, inkludert de som feilet.
    pub fn antall_totalt(&self) -> usize {
        self.antall_summert + self.feil.len()
    }
}

pub fn summer_verdier(kolonne_nr: usize, filstier: &[String]) -> Result<Summering, String> {
    let antall_filer = filstier.len();

    let pb = ProgressBar::new(antall_filer as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] {bar:30.green/black} {pos}/{len} ({percent}%)",
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
                // Feilmeldingene fra finn_sum_for_fil inneholder allerede filstien.
                let res = finn_sum_for_fil(kolonne_nr, filsti).map_err(|e| e.to_string());
                pb_clone.inc(1);
                log_minnebruk(filsti);
                res
            })
            .collect()
    });
    drop(pool);

    let mut headere: HashMap<String, Vec<String>> = HashMap::with_capacity(8);
    let mut totalsum = 0.0f64;
    let mut antall_summert = 0;
    let mut feil: Vec<String> = Vec::new();

    for result in results {
        match result {
            Ok((name, file, sum)) => {
                totalsum += sum;
                antall_summert += 1;
                headere.entry(name).or_default().push(file);
            }
            Err(e) => feil.push(e),
        }
    }

    pb.finish_and_clear();
    log_minnebruk("ferdig summering");

    Ok(Summering {
        antall_summert,
        sum: totalsum,
        headere,
        feil,
    })
}

fn finn_sum_for_fil(
    kolonne_nr: usize,
    filsti: &str,
) -> Result<(String, String, f64), Box<dyn Error>> {
    let filnavn = filsti
        .split('/')
        .next_back()
        .unwrap_or("ukjent")
        .to_string();

    let reader = aapne_fil_som_leser(filsti)?;
    let mut linjer = reader.lines();

    // Leser kun første linje for å detektere header — ingen full fil-allokering.
    let første_linje = match linjer.next() {
        Some(Ok(l)) => l,
        Some(Err(e)) => {
            return Err(Box::from(format!(
                "Lese-feil i '{}' linje 1: {}",
                filsti, e
            )));
        }
        None => return Ok((String::new(), filnavn, 0.0)),
    };

    // Sjekker om første element i fila IKKE er et tall.
    let har_header = første_linje
        .split(';')
        .nth(kolonne_nr)
        .is_none_or(|v| v.trim().parse::<f64>().is_err());

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
        summer_rad(&første_linje, 1, kolonne_nr, filsti, &mut beløp_fil)?;
    }
    // første_linje er ikke lenger nødvendig; slipp den før resten streames.
    drop(første_linje);

    // Strøm linje for linje: kun én String-allokering av gangen, slippes etter hver iterasjon.
    // Linjenummer er 1-basert; første linje er allerede lest.
    for (linjenr, linje_result) in (2..).zip(linjer) {
        let linje = linje_result
            .map_err(|e| format!("Lese-feil i '{}' linje {}: {}", filsti, linjenr, e))?;
        summer_rad(&linje, linjenr, kolonne_nr, filsti, &mut beløp_fil)?;
    }

    Ok((header_str, filnavn, beløp_fil))
}

/// Akkumulerer én semikolon-separert rad inn i `sum`. Bruker `nth()` på
/// split-iteratoren for å unngå å allokere en Vec av kolonnesegmenter.
fn summer_rad(
    linje: &str,
    linjenr: usize,
    kolonne_nr: usize,
    filsti: &str,
    sum: &mut f64,
) -> Result<(), Box<dyn Error>> {
    let kolonne_str = match linje.split(';').nth(kolonne_nr) {
        Some(v) => v.trim(),
        None => {
            // Kolonner er 0-indeksert, samme som i CLI-argumentet og `pcli header`.
            let antall_kolonner = linje.split(';').count();
            return Err(Box::from(format!(
                "Kolonne {} finnes ikke i '{}' linje {} (raden har {} kolonner, 0-{}).",
                kolonne_nr,
                filsti,
                linjenr,
                antall_kolonner,
                antall_kolonner - 1
            )));
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
            "Kunne ikke parse verdi '{}' som et tall i '{}' linje {}.",
            kolonne_str, filsti, linjenr
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
                    if let Some(kb) = linje
                        .split_whitespace()
                        .nth(1)
                        .and_then(|kb_str| kb_str.parse::<u64>().ok())
                    {
                        eprintln!("[mem] RSS: {} MB | {}", kb / 1024, kontekst);
                    }
                    break;
                }
            }
        }
    }
    #[cfg(not(target_os = "linux"))]
    let _ = kontekst;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn skriv(mappe: &std::path::Path, navn: &str, innhold: &str) -> String {
        let sti = mappe.join(navn);
        std::fs::File::create(&sti)
            .unwrap()
            .write_all(innhold.as_bytes())
            .unwrap();
        sti.to_string_lossy().to_string()
    }

    #[test]
    fn summerer_uten_feil() {
        let mappe = tempdir().unwrap();
        let fil = skriv(mappe.path(), "a.csv", "id;beløp\n1;10.5\n2;\n3;4.5\n");
        let s = summer_verdier(1, &[fil]).unwrap();
        assert_eq!(s.sum, 15.0);
        assert!(s.feil.is_empty());
    }

    #[test]
    fn rapporterer_fil_som_feiler() {
        let mappe = tempdir().unwrap();
        let ok = skriv(mappe.path(), "ok.csv", "id;beløp\n1;10\n");
        let feil = skriv(mappe.path(), "feil.csv", "id;beløp\n1;x\n");
        let s = summer_verdier(1, &[ok, feil]).unwrap();
        assert_eq!(s.sum, 10.0);
        assert_eq!(s.antall_summert, 1);
        assert_eq!(s.antall_totalt(), 2);
        assert_eq!(s.feil.len(), 1);
        assert!(s.feil[0].contains("'x'"), "{}", s.feil[0]);
        assert!(s.feil[0].contains("linje 2"), "{}", s.feil[0]);
    }

    #[test]
    fn manglende_kolonne_bruker_0_indeksert_nummer() {
        let mappe = tempdir().unwrap();
        let fil = skriv(mappe.path(), "a.csv", "id;beløp\n1;2\n");
        let s = summer_verdier(5, &[fil]).unwrap();
        assert_eq!(s.antall_summert, 0);
        assert!(
            s.feil[0].starts_with("Kolonne 5 finnes ikke"),
            "{}",
            s.feil[0]
        );
        assert!(s.feil[0].contains("0-1"), "{}", s.feil[0]);
    }

    #[test]
    fn filsti_står_kun_én_gang_i_feilmelding() {
        let mappe = tempdir().unwrap();
        let fil = skriv(mappe.path(), "a.csv", "id;beløp\n1;x\n");
        let s = summer_verdier(1, std::slice::from_ref(&fil)).unwrap();
        assert_eq!(s.feil[0].matches(&fil).count(), 1, "{}", s.feil[0]);
    }

    #[test]
    fn fil_uten_header_summerer_første_linje() {
        let mappe = tempdir().unwrap();
        let fil = skriv(mappe.path(), "a.csv", "1;10\n2;5\n");
        let s = summer_verdier(1, &[fil]).unwrap();
        assert_eq!(s.sum, 15.0);
        assert!(s.headere.contains_key(""));
    }

    #[test]
    fn ustøttet_filtype_gir_feil() {
        let mappe = tempdir().unwrap();
        let fil = skriv(mappe.path(), "a.txt", "1;10\n");
        let s = summer_verdier(1, &[fil]).unwrap();
        assert_eq!(s.antall_summert, 0);
        assert_eq!(s.feil.len(), 1);
    }
}
