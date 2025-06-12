use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use crate::kommandoer::utils;

pub fn rutinefil_valider(filsti: &str) {
    const VARIABLER_KEY: &str = "variabler";
    println!("Kjører validering på filen '{}'.", &filsti);

    if utils::filen_ikke_er_json(filsti) {
        return;
    }

    let mut definerte_variabler: HashSet<String> = HashSet::new();
    let mut udefinerte_variabler = Vec::new();
    let mut definert_men_ikke_brukte_variabler = Vec::new();
    let mut manglende_variabler = Vec::new();
    let mut inneholder_feil = false;

    let filinnhold = match fs::read_to_string(filsti) {
        Ok(innhold) => innhold,
        Err(e) => {
            eprintln!("Klarte ikke lese filen: {}", e);
            return;
        }
    };

    let json: Value = match serde_json::from_str(&filinnhold) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Ikke gyldig JSON format: {}", e);
            return;
        }
    };

    let brukte_variabler = finn_brukte_variabler(&filinnhold);

    if let Some(variabler) = json.get(VARIABLER_KEY).and_then(Value::as_object) {
        for (key, value) in variabler {
            definerte_variabler.insert(key.to_string());

            match value {
                Value::String(s)=> {
                    if s.starts_with("<") || s.ends_with(">") {
                        udefinerte_variabler.push(format!("{}: {}", key, s));
                        inneholder_feil = true;
                    }
                },
                Value::Array(arr) => {

                   if arr.is_empty() || (arr.len() == 1 && arr.get(0).and_then(Value::as_str).map_or(false, |s| s.starts_with('<') || s.ends_with('>')))
                    {
                        udefinerte_variabler.push(format!("{}: {}", key, serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string())));
                        inneholder_feil = true;
                    }
                },
                _ => {}
            }
        }
    }

    // Sjekker for brukte variabler som ikke er definert
    for var in &brukte_variabler {
        if !definerte_variabler.contains(var) {
            manglende_variabler.push(var.clone());
            inneholder_feil = true;
        }
    }
    
    // Sjekker for definerte variabler som ikke er brukt
    for var in &definerte_variabler {
        if !brukte_variabler.contains(var) {
            definert_men_ikke_brukte_variabler.push(var.clone());
            inneholder_feil = true;
        }
    }

    if !inneholder_feil {
        println!("✅ Ingen feil funnet i rutinefilen '{}'.", filsti);
    }

    match manglende_variabler.as_slice() {
        [] => {} // Ingen manglende variabler
        variabler => {
            rapporter_valideringsfeil(
                "❌  Følgende brukte variabler mangler i 'variabler', filen vil ikke fungere:",
                variabler,
            );
        }
    }
    
    match udefinerte_variabler.as_slice() {
        [] => {} // Ingen udefinerte variabler
        variabler => {
            rapporter_valideringsfeil(
                "❓  Følgende variabler har ikke blitt utfylt:",
                variabler,
            );
        }
    }

    match definert_men_ikke_brukte_variabler.as_slice() {
        [] => {} // Ingen definerte variabler som ikke er brukt
        variabler => {
            rapporter_valideringsfeil(
                "⚠️  Følgende variabler er definert men ikke brukt i rutinefilen:",
                variabler,
            );
        }
    }
    println!();

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
