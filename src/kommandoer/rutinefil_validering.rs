use serde_json::Value;
use std::collections::HashSet;
use crate::kommandoer::io_utils;

pub struct ValidationResult {
    pub manglende_variabler: Vec<String>,
    pub udefinerte_variabler: Vec<String>,
    pub ubrukte_variabler: Vec<String>,
}

pub fn rutinefil_valider(filsti: &str) {
    println!("Validering filen '{}'.", filsti);

    let filinnhold = match io_utils::les_filinnhold(filsti) {
        Ok(innhold) => innhold,
        Err(e) => {
            eprintln!("Feil ved lesing av fil: {}", e);
            println!();
            return;
        }
    };

    match validering_innhold_rutinefil(&filinnhold) {
        Ok(resultat) => {
            if resultat.manglende_variabler.is_empty()
                && resultat.udefinerte_variabler.is_empty()
                && resultat.ubrukte_variabler.is_empty()
            {
                println!("✅ Validering fullført, ingen feil funnet.");
            } else {
                if !resultat.manglende_variabler.is_empty() {
                    rapporter_valideringsfeil(
                        "❌ Følgende brukte variabler mangler i 'variabler', filen vil ikke fungere:",
                        &resultat.manglende_variabler,
                    );
                }
                if !resultat.udefinerte_variabler.is_empty() {
                    rapporter_valideringsfeil(
                        "❓ Følgende variabler har ikke blitt utfylt:",
                        &resultat.udefinerte_variabler,
                    );
                }
                if !resultat.ubrukte_variabler.is_empty() {
                    rapporter_valideringsfeil(
                        "⚠️ Følgende variabler er definert, men ikke brukt i filen:",
                        &resultat.ubrukte_variabler,
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Validering feilet: {}", e);
        }
    }
        println!();
}


pub fn validering_innhold_rutinefil(filinnhold: &str) -> Result<ValidationResult, String> {
    // Nøkkelen som brukes for variabler i JSON-filen
    const VARIABLER_KEY: &str = "variabler";

    let json = io_utils::parse_json(filinnhold);
    if json.is_err() {
        return Err(format!("Klarte ikke parse JSON: {}", json.unwrap_err()));
    }

    let mut definerte_variabler: HashSet<String> = HashSet::new();
    let mut udefinerte_variabler = Vec::new();
    let mut ubrukte_variabler = Vec::new();
    let mut manglende_variabler = Vec::new();

    let brukte_variabler = finn_brukte_variabler(&filinnhold);

    if let Some(variabler) = json.unwrap().get(VARIABLER_KEY).and_then(Value::as_object) {
        for (key, value) in variabler {
            definerte_variabler.insert(key.to_string());

            match value {
                Value::String(s) => {
                    if s.starts_with("<") || s.ends_with(">") {
                        udefinerte_variabler.push(format!(
                            "{}: {}",
                            key,
                            s
                        ));
                    }
                }
                Value::Array(arr) => {
                    if arr.is_empty()
                        || (arr.len() == 1
                        && arr
                        .get(0)
                        .and_then(Value::as_str)
                        .map_or(false, |s| s.starts_with('<') || s.ends_with('>')))
                    {
                        udefinerte_variabler.push(format!(
                            "{}: {}",
                            key,
                            serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string())
                        ));
                    }
                }
                _ => {}
            }
        }
    } else {
        Err("Ingen 'variabler' nøkkel funnet i JSON-filen.".to_string())?;
    }

    // Sjekker for brukte variabler som ikke er definert
    for var in &brukte_variabler {
        if !definerte_variabler.contains(var) {
            manglende_variabler.push(var.clone());
        }
    }

    // Sjekker for definerte variabler som ikke er brukt
    for var in &definerte_variabler {
        if !brukte_variabler.contains(var) {
            ubrukte_variabler.push(var.clone());
        }
    }

    Ok(ValidationResult {
        manglende_variabler,
        udefinerte_variabler,
        ubrukte_variabler
    })
}

fn finn_brukte_variabler(s: &str) -> HashSet<String> {
    let mut brukte = HashSet::new();
    let mut start = 0;

    while let Some(pos) = s[start..].find("${") {
        if let Some(slutt) = s[start + pos + 2..].find('}') {
            let var_name = &s[start + pos + 2..start + pos + 2 + slutt];
            brukte.insert(var_name.to_string());
            start += pos + 2 + slutt + 1;
        } else {
            break;
        }
    }
    brukte
}

fn rapporter_valideringsfeil(feilmelding: &str, vars: &[String]) {
    if vars.is_empty() {
        return;
    }
    println!("{}", feilmelding);
    for var in vars {
        println!("   - {}", var);
    }
}
