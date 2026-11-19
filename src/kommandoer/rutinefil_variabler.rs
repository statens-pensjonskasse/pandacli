use std::fs;

use serde_json::Value;

/// Returnerer `false` hvis fila ikke kunne leses eller parses.
pub fn rutinefil_variabler(filsti: &str) -> bool {
    const VARIABLER_KEY: &str = "variabler";

    let filinnhold = match fs::read_to_string(filsti) {
        Ok(innhold) => innhold,
        Err(e) => {
            eprintln!("Klarte ikke lese filen: {}", e);
            return false;
        }
    };

    let json: Value = match serde_json::from_str(&filinnhold) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Ikke gyldig JSON format: {}", e);
            return false;
        }
    };

    let mut definerte_variabler = Vec::new();

    if let Some(variabler) = json.get(VARIABLER_KEY).and_then(Value::as_object) {
        for (key, value) in variabler {
            definerte_variabler.push(format!("{} = {}", key, value));
        }
    }

    match definerte_variabler.as_slice() {
        [] => {
            println!("Ingen variabler er definert i rutinefilen '{}'.", filsti);
        }
        variabler => {
            println!("Følgende variabler er definert i '{}'.", filsti);
            for v in variabler {
                println!("   - {}", v);
            }
        }
    }
    true
}
