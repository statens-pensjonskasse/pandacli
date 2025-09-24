use std::fs::File;
use std::io::{self, BufRead};

pub fn operasjoner(filsti: &str) -> Result<String, String> {

    let file = File::open(filsti).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut remaining: u32 = 0;
    let mut max: Option<u32> = None;

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if let Some(num) = line.strip_prefix("Handling: ") {
            if let Ok(val) = num.trim().parse::<u32>() {
                max = max.or_else(|| Some(val));
                remaining += 1;
            }
        }
    }

    if remaining == 0 {
        println!("Ingen handlinger funnet i filen.");
        return Ok(("Ingen handlinger funnet i filen").to_string());
    }

    let total = max.unwrap();
    let percent = (remaining * 100 / total) ;
    
    if percent == 100 {
        return Ok("Fremdrift: Ferdig, alle instruksjoner kjørt.".to_string());
    }
    
    Ok(format!("Fremdrift: {percent}% ferdig ({remaining} av {total})"))
}
