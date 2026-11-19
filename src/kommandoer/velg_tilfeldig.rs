use rand::Rng;

/// Returnerer `false` hvis det ikke finnes verdier å velge fra.
pub fn velg_tilfeldig(verdier: Vec<String>) -> bool {
    if verdier.is_empty() {
        eprintln!("Ingen verdier å velge fra.");
        return false;
    }

    let tilfeldig_indeks =  rand::rng().random_range(0..verdier.len());
    let valgt_verdi = &verdier[tilfeldig_indeks];

    println!("Valgt verdi: {}", valgt_verdi);
    true
}
