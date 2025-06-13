
pub fn summer_verdier(kolonne_nr: &i8, filstier: &Vec<String>) -> Result<f64, String> {
    let mut total_sum: f64 = 0.0;

    for filsti in filstier {
        let innhold = std::fs::read_to_string(filsti)
            .map_err(|e| format!("Klarte ikke lese filen '{}': {}", filsti, e))?;

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
    println!("{}", total_sum);
    Ok(total_sum)
}