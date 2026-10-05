//! Testrigg som kjører `valider` mot fikstur-samlingen i `tests/resources/`.
//!
//! - `tests/resources/rutinefiler_gyldige/`: filer som skal validere uten feil.
//! - `tests/resources/rutinefiler_med_feil/`: filer med kjente problemer. Hver fil har
//!   en sidecar `<navn>.forventet.json` som beskriver forventet resultat.
//!
//! Se `tests/resources/README.md` for formatet på sidecar-filene.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::kommandoer::rutinefil_validering::validering_innhold_rutinefil;

#[derive(Deserialize)]
struct Forventet {
    #[serde(default)]
    forventede_kategorier: Vec<String>,
    #[serde(default)]
    kjent_mangel: bool,
}

fn resources_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("resources")
}

/// Alle `*.json`-filer i en mappe, utenom sidecar-filene (`*.forventet.json`).
fn rutinefiler(dir: &Path) -> Vec<PathBuf> {
    let mut filer: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Klarte ikke lese {:?}: {}", dir, e))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .filter(|p| !p.to_string_lossy().ends_with(".forventet.json"))
        .collect();
    filer.sort();
    filer
}

fn sidecar_sti(fil: &Path) -> PathBuf {
    let stamme = fil.file_stem().unwrap().to_str().unwrap();
    fil.with_file_name(format!("{}.forventet.json", stamme))
}

#[test]
fn gyldige_rutinefiler_gir_ingen_feil() {
    let dir = resources_dir().join("rutinefiler_gyldige");
    let filer = rutinefiler(&dir);
    if filer.is_empty() {
        // Korpuset bygges opp med ekte eksempler over tid. Tom mappe er ok.
        return;
    }

    for fil in filer {
        let innhold = fs::read_to_string(&fil).unwrap();
        // streng_metainfo=true: fiksturene er kuraterte, så metainfo.mal-sjekken skal kjøre.
        let resultat = validering_innhold_rutinefil(&innhold, fil.to_str().unwrap(), true)
            .unwrap_or_else(|e| panic!("{:?}: parsing feilet: {}", fil, e));
        assert!(
            resultat.har_ingen_feil(),
            "{:?} skulle være gyldig, men fikk feil i kategoriene: {:?}",
            fil,
            resultat.kategorier_med_feil()
        );
    }
}

#[test]
fn rutinefiler_med_feil_oppdages() {
    let dir = resources_dir().join("rutinefiler_med_feil");
    let filer = rutinefiler(&dir);
    if filer.is_empty() {
        // Korpuset bygges opp med ekte eksempler over tid. Tom mappe er ok.
        return;
    }

    for fil in filer {
        let sidecar = sidecar_sti(&fil);
        assert!(
            sidecar.exists(),
            "mangler forventningsfil {:?} for {:?}",
            sidecar,
            fil
        );
        let forventet: Forventet = serde_json::from_str(&fs::read_to_string(&sidecar).unwrap())
            .unwrap_or_else(|e| panic!("{:?}: ugyldig sidecar: {}", sidecar, e));

        let innhold = fs::read_to_string(&fil).unwrap();
        // streng_metainfo=true: fiksturene er kuraterte, så metainfo.mal-sjekken skal kjøre.
        let resultat = validering_innhold_rutinefil(&innhold, fil.to_str().unwrap(), true)
            .unwrap_or_else(|e| panic!("{:?}: parsing feilet: {}", fil, e));
        let funnet = resultat.kategorier_med_feil();

        if forventet.kjent_mangel {
            // Kjent mangel: dagens validering fanger ikke feilen. De forventede
            // kategoriene skal fortsatt være tomme. Testen flipper (feiler) når
            // en sjekk legges til, så vi husker å oppdatere sidecar-en.
            for kategori in &forventet.forventede_kategorier {
                assert!(
                    !funnet.contains(&kategori.as_str()),
                    "{:?} er merket som kjent_mangel, men kategori '{}' fanges nå. \
                     Sett kjent_mangel=false i {:?}.",
                    fil,
                    kategori,
                    sidecar
                );
            }
        } else {
            assert!(
                !forventet.forventede_kategorier.is_empty(),
                "{:?}: må angi minst én forventet kategori (eller sette kjent_mangel=true)",
                sidecar
            );
            for kategori in &forventet.forventede_kategorier {
                assert!(
                    funnet.contains(&kategori.as_str()),
                    "{:?} skulle gi feil i '{}', men fikk kategoriene: {:?}",
                    fil,
                    kategori,
                    funnet
                );
            }
        }
    }
}
