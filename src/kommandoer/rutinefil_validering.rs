use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn valider_rutinefil(filsti: &str) {
    const VARIABLER_KEY: &str = "variabler";
    if filen_ikke_er_json(filsti) {
        return;
    }

    let mut definerte_variabler = HashSet::new();
    let mut udefinerte_variabler = Vec::new();
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

    // Extract defined variables and check for undefined values
    if let Some(variabler) = json.get(VARIABLER_KEY).and_then(Value::as_object) {
        for (key, value) in variabler {
            definerte_variabler.insert(key.to_string());
            if let Some(str_value) = value.as_str() {
                if str_value.contains('<') {
                    udefinerte_variabler.push(format!("{}: {}", key, str_value));
                    inneholder_feil = true;
                }
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
    
    if !inneholder_feil {
        println!("✅ Ingen feil funnet i rutinefilen '{}'.", filsti);
    } else {
        println!("🚫 Feil funnet i rutinefilen '{}'.", &filsti)
    }

    match manglende_variabler.as_slice() {
        [] => {} // Ingen manglende variabler
        variabler => {
            rapporter_valideringsfeil(
                "❌ Følgende brukte variabler mangler i 'variabler', filen vil ikke fungere:",
                variabler,
            );
        }
    }

    if brukte_variabler.is_empty() {
        println!("🚫 Ingen variabler brukt i rutinefilen '{}'.", filsti);
    }

    if definerte_variabler.is_empty() {
        println!("🚫 Ingen variabler definert i rutinefilen '{}'.", filsti);
    }
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

fn filen_ikke_er_json(filsti: &str) -> bool {
    let path = Path::new(filsti);
    if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
        eprintln!("🚫 Error: Filen '{}' har ikke .json filending.", filsti);
        return true;
    }
    false
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
