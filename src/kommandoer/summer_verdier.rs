use std::fs::File;
use std::io::Read;
use flate2::read::GzDecoder;

pub fn summer_verdier(kolonne_nr: &i8, filstier: &Vec<String>) -> Result<f64, String> {
    let mut total_sum: f64 = 0.0;
    let mut antall_filer: u16 = 0;
    
    for filsti in filstier {
        antall_filer += 1;
        
        let mut innhold = String::new();
        if filsti.ends_with(".gz") {
            let tar_gz = File::open(filsti)
                .map_err(|e| format!("Klarte ikke åpne filen '{}': {}", filsti, e))?;
           
            let mut gz_decoder = GzDecoder::new(tar_gz);
            gz_decoder.read_to_string(&mut innhold)
                .map_err(|e| format!("Klarte ikke lese innholdet fra gz-filen '{}': {}", filsti, e))?;
        } 
        else if filsti.ends_with(".csv") {
            innhold = std::fs::read_to_string(filsti)
                .map_err(|e| format!("Klarte ikke lese filen '{}': {}", filsti, e))?;
        } else {
            return Err(format!(
                "Filen '{}' er ikke et støttet format.",
                filsti
            ));
        }

        let rader: Vec<&str> = innhold.lines().collect();
        if rader.is_empty() {
            continue; // Hopp over tomme filer
        }

        // Hopper over header-raden. Burde egentlig sjekke om det er header eller ikke
        for rad in rader.iter().skip(1) {
            let kolonner: Vec<&str> = rad.split(';').collect();

            if (kolonne_nr < &0) || (kolonne_nr >= &(kolonner.len() as i8)) {
                return Err(format!(
                    "Kolonne {} finnes ikke i filen '{}'.",
                    kolonne_nr + 1,
                    filsti
                ));
            }


            if let Ok(verdi) = kolonner[*kolonne_nr as usize].trim().parse::<f64>() {
                total_sum += verdi;
            } else {
                return Err(format!(
                    "Kunne ikke parse verdi '{}' som et tall i filen '{}'.",
                    kolonner[*kolonne_nr as usize], filsti
                ));
            }
        }
    }

    //Håndtere resultatet fra summeringen og ikke bare printe?
    println!("{} filer, sum: {}", antall_filer, total_sum);
    Ok(total_sum)
}