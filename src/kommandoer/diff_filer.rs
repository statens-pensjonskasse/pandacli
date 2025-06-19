


pub fn diff_filer(venstre: Vec<String>, høyre: Vec<String>, _ignorert: Vec<usize>) -> Result<(), String> {
    if venstre.len() != høyre.len() {
        return Err("Antall filer i venstre og høyre side må være likt.".to_string());
    }

    for (v, h) in venstre.iter().zip(høyre.iter()) {
        let v_innhold = std::fs::read_to_string(v)
            .map_err(|e| format!("Klarte ikke lese filen '{}': {}", v, e))?;
        let h_innhold = std::fs::read_to_string(h)
            .map_err(|e| format!("Klarte ikke lese filen '{}': {}", h, e))?;

        if v_innhold != h_innhold {
            println!("Forskjell funnet mellom '{}' og '{}'", v, h);
        } else {
            println!("Ingen forskjeller mellom '{}' og '{}'", v, h);
        }
    }

    Ok(())
}