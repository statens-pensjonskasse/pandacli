use std::fs;

use serde_json::Value;
use crate::kommandoer::utils;

pub fn rutinefil_variabler(filsti: &str) {
    const VARIABLER_KEY: &str = "variabler";
    if utils::filen_ikke_er_json(filsti) {
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
