use std::error::Error;
use std::fs::File;
use flate2::read::GzDecoder;
use std::io::Read;

pub fn header(filsti: &String) -> Result<Vec<String>, String> {

    let innhold = les_innhold_fil(filsti).ok().unwrap();
    let rader: Vec<&str> = innhold.lines().collect();

    if rader.is_empty() {
        return Ok(vec![format!("Ingen rader i filen")]);
    }

    let headere: Vec<&str> = rader[0].split(";").collect();
    let indekserte_headere: Vec<String> = headere
        .iter()
        .enumerate()
        .map(|(index, &h)| format!("{}: {}", index, h))
        .collect();

    Ok(indekserte_headere)
}

fn les_innhold_fil(filsti: &String) -> Result<String, Box<dyn Error>> {
    let mut innhold = String::new();

    if filsti.ends_with(".gz") {
        let tar_gz = File::open(filsti)
            .map_err(|e| format!("Klarte ikke åpne filen '{}': {}", filsti, e))?;

        let mut gz_decoder = GzDecoder::new(tar_gz);
        gz_decoder.read_to_string(&mut innhold).map_err(|e| {
            format!(
                "Klarte ikke lese innholdet fra gz-filen '{}': {}",
                filsti, e
            )
        })?;
    } else if filsti.ends_with(".csv") {
        innhold = std::fs::read_to_string(filsti)
            .map_err(|e| format!("Klarte ikke lese filen '{}': {}", filsti, e))?;
    } else {
        return Err(Box::from(format!(
            "Filen '{}' er ikke et støttet format.",
            filsti
        )));
    }

    Ok(innhold)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use flate2::write::GzEncoder;
    use flate2::Compression;

    fn lag_test_fil(filsti: &str, innhold: &str) -> String {
        let mut file = File::create(filsti).unwrap();
        file.write_all(innhold.as_bytes()).unwrap();
        filsti.to_string()
    }

    #[test]
    fn test_fil_med_header_og_ekstra_innhold() {
        let mappe = tempdir().unwrap();
        let mappesti = mappe.path().join("test_med_header.csv");
        let sti = mappesti.to_string_lossy().to_string();

        let innhold = "personId;avtale;bokføringsperiode\n12314123;928219;2020501\n519929213;123141;202501";
        lag_test_fil(&sti, innhold);

        let resultat = header(&sti).unwrap();

        assert_eq!(resultat, vec![
            "0: personId".to_string(),
            "1: avtale".to_string(),
            "2: bokføringsperiode".to_string()
        ]);
    }

    #[test]
    fn test_fil_med_kun_data() {
        let mappe = tempdir().unwrap();
        let mappesti = mappe.path().join("data_only.csv");
        let sti = mappesti.to_string_lossy().to_string();

        let innhold = "12314123;928219;2020501\n519929213;123141;202501";
        lag_test_fil(&sti, innhold);

        let resultat = header(&sti).unwrap();

        assert_eq!(resultat, vec![
            "0: 12314123".to_string(),
            "1: 928219".to_string(),
            "2: 2020501".to_string()
        ]);
    }

    #[test]
    fn test_tom_fil() {
        let mappe = tempdir().unwrap();
        let mappesti = mappe.path().join("empty.csv");
        let sti = mappesti.to_string_lossy().to_string();

        // Create an empty file
        lag_test_fil(&sti, "");

        let resultat = header(&sti).unwrap();

        assert_eq!(resultat, vec!["Ingen rader i filen".to_string()]);
    }

    #[test]
    fn test_gzipped_fil() {
        let mappe = tempdir().unwrap();
        let mappesti = mappe.path().join("compressed.gz");
        let sti = mappesti.to_string_lossy().to_string();

        let innhold = "personId;avtale;periode\n12314123;928219;2020501\n519929213;123141;202501";

        // Create gzipped content
        let fil = File::create(&sti).unwrap();
        let mut enkoder = GzEncoder::new(fil, Compression::default());
        enkoder.write_all(innhold.as_bytes()).unwrap();
        enkoder.finish().unwrap();

        let resultat = header(&sti).unwrap();

        assert_eq!(resultat, vec![
            "0: personId".to_string(),
            "1: avtale".to_string(),
            "2: periode".to_string()
        ]);
    }
}