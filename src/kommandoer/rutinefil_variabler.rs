use std::fs;
use std::path::Path;
use serde_json::Value;

pub fn rutinefil_variabler(filsti: &str) {
    const VARIABLER_KEY: &str = "variabler";
    if filen_ikke_er_json(filsti) {
        return;
    }
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

    let mut definerte_variabler = Vec::new();


    if let Some(variabler) = json.get(VARIABLER_KEY).and_then(Value::as_object) {
        for (key, value) in variabler {
            definerte_variabler.push(format!("{} = {}", key, value));
        }
    }

    match definerte_variabler.as_slice() {  
        [] => {
            println!("Ingen variabler er definert i rutinefilen '{}'.", filsti);
            return;
        },
        variabler =>{
            println!("Følgende variabler er definert i '{}'.", filsti);
            for v in variabler {
            println!("   - {}", v);
            }
        }
    }
}

fn filen_ikke_er_json(filsti: &str) -> bool {
    let path = Path::new(filsti);
    if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
        eprintln!("🚫 Error: Filen '{}' har ikke .json filending.", filsti);
        return true;
    }
    false
}
