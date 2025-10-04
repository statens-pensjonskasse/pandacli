use std::fs::File;
use std::io::{self, BufRead};

pub fn operasjoner(filsti: &str) -> Result<String, String> {

    let file = File::open(filsti).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut resterende: u32 = 0;
    let mut maks: Option<u32> = None;

    for linjer in reader.lines() {
        let linje = linjer.map_err(|e| e.to_string())?;
        if let Some(num) = linje.strip_prefix("Handling: ") {
            if let Ok(val) = num.trim().parse::<u32>() {
                maks = maks.or_else(|| Some(val));
                resterende += 1;
            }
        }
    }

    if resterende == 0 {
        println!("Ingen handlinger funnet i filen.");
        return Ok(("Ingen handlinger funnet i filen").to_string());
    }

    let totalt = maks.unwrap();
    let prosent = resterende * 100 / totalt;
    
    if prosent == 100 {
        return Ok("Fremdrift: Ferdig, alle instruksjoner kjørt.".to_string());
    }
    
    Ok(format!("Fremdrift: {prosent}% ferdig ({resterende} av {totalt})"))
}
