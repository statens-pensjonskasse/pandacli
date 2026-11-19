use std::fs::File;
use std::io::{self, BufRead};

pub fn operasjoner(filsti: &str) -> Result<String, String> {
    let file = File::open(filsti).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut resterende: u32 = 0;
    let mut maks: Option<u32> = None;
    let mut handling: Option<String> = None;

    for linjer in reader.lines() {
        let linje = linjer.map_err(|e| e.to_string())?;

        if let Some(rest) = linje.find(r#""handling":""#).and_then(|pos| {
            let start = pos + r#""handling":""#.len();
            linje[start..]
                .find('"')
                .map(|end| &linje[start..start + end])
        }) {
            handling = Some(rest.trim().to_string());
        }

        if let Some(num) = linje.strip_prefix("Handling: ") {
            if let Ok(val) = num.trim().parse::<u32>() {
                maks = maks.or_else(|| Some(val));
                resterende += 1;
            }
        }

        if let Some(_error) = linje.strip_prefix("Operasjon feilet med: ") {
            // Feilen kan stå før første "handling" i loggen.
            return Err(format!(
                "Kjøringen har feilet for {}",
                handling.as_deref().unwrap_or("ukjent handling")
            ));
        }
    }

    if resterende == 0 {
        println!("Ingen handlinger funnet i filen.");
        return Ok(("Ingen handlinger funnet i filen").to_string());
    }

    // maks er alltid satt når resterende > 0, men totalt kan være 0 i loggen.
    let totalt = maks.unwrap_or(0);
    if totalt == 0 {
        return Err("Ugyldig logg: totalt antall handlinger er 0.".to_string());
    }
    let prosent = resterende * 100 / totalt;

    if prosent == 100 {
        return Ok("Ferdig: Alle instruksjoner kjørt".to_string());
    }

    Ok(format!(
        "Fremdrift: {prosent}% ferdig, kjører {} ({resterende} av {totalt})", handling.as_deref().unwrap_or("ukjent")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn logg(innhold: &str) -> NamedTempFile {
        let mut fil = NamedTempFile::new().unwrap();
        fil.write_all(innhold.as_bytes()).unwrap();
        fil
    }

    fn kjør(innhold: &str) -> Result<String, String> {
        let fil = logg(innhold);
        operasjoner(fil.path().to_str().unwrap())
    }

    #[test]
    fn feil_før_første_handling_gir_feil_uten_panikk() {
        let r = kjør("Operasjon feilet med: noe gikk galt\n");
        assert_eq!(r, Err("Kjøringen har feilet for ukjent handling".to_string()));
    }

    #[test]
    fn feil_etter_handling_navngir_handlingen() {
        let r = kjør("Handling: 3\n{\"handling\":\"pa_res_ba_02\"}\nOperasjon feilet med: x\n");
        assert_eq!(r, Err("Kjøringen har feilet for pa_res_ba_02".to_string()));
    }

    #[test]
    fn totalt_null_gir_feil_uten_panikk() {
        assert!(kjør("Handling: 0\n").is_err());
    }

    #[test]
    fn rapporterer_fremdrift() {
        let r = kjør("Handling: 4\n{\"handling\":\"pa_res_ba_02\"}\nHandling: 3\n").unwrap();
        assert_eq!(r, "Fremdrift: 50% ferdig, kjører pa_res_ba_02 (2 av 4)");
    }

    #[test]
    fn ingen_handlinger() {
        assert_eq!(kjør("").unwrap(), "Ingen handlinger funnet i filen");
    }

    #[test]
    fn manglende_fil_gir_feil() {
        assert!(operasjoner("finnes/ikke.log").is_err());
    }
}
