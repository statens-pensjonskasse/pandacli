use rand::Rng;

pub fn velg_tilfeldig(verdier: Vec<String>) {
    if verdier.is_empty() {
        println!("Ingen verdier å velge fra.");
        return;
    }

    let tilfeldig_indeks =  rand::rng().random_range(0..verdier.len());
    let valgt_verdi = &verdier[tilfeldig_indeks];

    println!("Valgt verdi: {}", valgt_verdi);
}