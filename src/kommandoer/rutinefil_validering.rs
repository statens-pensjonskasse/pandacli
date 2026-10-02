use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

use crate::kommandoer::io_utils;
use crate::kommandoer::rutinefil_katalog;

pub struct ValidationResult {
    pub manglende_variabler: Vec<String>,
    pub udefinerte_variabler: Vec<String>,
    pub ubrukte_variabler: Vec<String>,
    pub ukjente_handlinger: Vec<String>,
    pub ukjente_funksjoner: Vec<String>,
    pub strukturfeil: Vec<String>,
    pub metainfo_feil: Vec<String>,
    pub versjonsfeil: Vec<String>,
    pub listevariabel_feil: Vec<String>,
    pub grunnlagsdatamappe_feil: Vec<String>,
}

impl ValidationResult {
    fn tom() -> Self {
        ValidationResult {
            manglende_variabler: Vec::new(),
            udefinerte_variabler: Vec::new(),
            ubrukte_variabler: Vec::new(),
            ukjente_handlinger: Vec::new(),
            ukjente_funksjoner: Vec::new(),
            strukturfeil: Vec::new(),
            metainfo_feil: Vec::new(),
            versjonsfeil: Vec::new(),
            listevariabel_feil: Vec::new(),
            grunnlagsdatamappe_feil: Vec::new(),
        }
    }

    fn ingen_feil(&self) -> bool {
        self.manglende_variabler.is_empty()
            && self.udefinerte_variabler.is_empty()
            && self.ubrukte_variabler.is_empty()
            && self.ukjente_handlinger.is_empty()
            && self.ukjente_funksjoner.is_empty()
            && self.strukturfeil.is_empty()
            && self.metainfo_feil.is_empty()
            && self.versjonsfeil.is_empty()
            && self.listevariabel_feil.is_empty()
            && self.grunnlagsdatamappe_feil.is_empty()
    }

    /// Alle feilkategorier med navn, i rapporteringsrekkefølge. Brukes av
    /// testriggen (`resources/`) til å slå opp forventede kategorier per fil.
    #[cfg(test)]
    pub(crate) fn kategorier(&self) -> [(&'static str, &Vec<String>); 10] {
        [
            ("manglende_variabler", &self.manglende_variabler),
            ("udefinerte_variabler", &self.udefinerte_variabler),
            ("ubrukte_variabler", &self.ubrukte_variabler),
            ("ukjente_handlinger", &self.ukjente_handlinger),
            ("ukjente_funksjoner", &self.ukjente_funksjoner),
            ("strukturfeil", &self.strukturfeil),
            ("metainfo_feil", &self.metainfo_feil),
            ("versjonsfeil", &self.versjonsfeil),
            ("listevariabel_feil", &self.listevariabel_feil),
            ("grunnlagsdatamappe_feil", &self.grunnlagsdatamappe_feil),
        ]
    }

    /// Navnene på kategoriene som har minst én feil.
    #[cfg(test)]
    pub(crate) fn kategorier_med_feil(&self) -> Vec<&'static str> {
        self.kategorier()
            .iter()
            .filter(|(_, feil)| !feil.is_empty())
            .map(|(navn, _)| *navn)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn har_ingen_feil(&self) -> bool {
        self.ingen_feil()
    }
}

pub fn rutinefil_valider(filsti: &str, streng_metainfo: bool) {
    println!(
        "Validering filen '{}' (katalog synket mot orkestrering v{}).",
        filsti,
        rutinefil_katalog::KATALOG_SYNKET_MOT_ORKESTRERING_VERSJON
    );

    let filinnhold = match io_utils::les_filinnhold(filsti) {
        Ok(innhold) => innhold,
        Err(e) => {
            eprintln!("Feil ved lesing av fil: {}", e);
            println!();
            return;
        }
    };

    match validering_innhold_rutinefil(&filinnhold, filsti, streng_metainfo) {
        Ok(resultat) => {
            if resultat.ingen_feil() {
                println!("✅ Validering fullført, ingen feil funnet.");
            } else {
                rapporter_valideringsfeil(
                    "❌ Følgende brukte variabler mangler i 'variabler', filen vil ikke fungere:",
                    &resultat.manglende_variabler,
                );
                rapporter_valideringsfeil(
                    "❌ Følgende operasjoner har ukjent 'handling' (støttes ikke av orkestreringa):",
                    &resultat.ukjente_handlinger,
                );
                rapporter_valideringsfeil(
                    "❌ Følgende #{...}-funksjoner støttes ikke av orkestreringa:",
                    &resultat.ukjente_funksjoner,
                );
                rapporter_valideringsfeil(
                    "❌ Følgende strukturfeil ble funnet:",
                    &resultat.strukturfeil,
                );
                rapporter_valideringsfeil(
                    "❌ Batch kjøres mot feil grunnlagsdataMappe (filene ble kopiert et annet sted):",
                    &resultat.grunnlagsdatamappe_feil,
                );
                rapporter_valideringsfeil(
                    "❌ Følgende listevariabler brukes feil (må stå som egen streng \"${variabel}\"):",
                    &resultat.listevariabel_feil,
                );
                rapporter_valideringsfeil(
                    "❌ Følgende versjonsverdier har ugyldig format:",
                    &resultat.versjonsfeil,
                );
                rapporter_valideringsfeil(
                    "❌ Følgende feil ble funnet i 'metainfo':",
                    &resultat.metainfo_feil,
                );
                rapporter_valideringsfeil(
                    "❓ Følgende variabler har ikke blitt utfylt:",
                    &resultat.udefinerte_variabler,
                );
                rapporter_valideringsfeil(
                    "⚠️ Følgende variabler er definert, men ikke brukt i filen:",
                    &resultat.ubrukte_variabler,
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Validering feilet: {}", e);
        }
    }
    println!();
}

pub fn validering_innhold_rutinefil(
    filinnhold: &str,
    filsti: &str,
    streng_metainfo: bool,
) -> Result<ValidationResult, String> {
    // Nøkkelen som brukes for variabler i JSON-filen
    const VARIABLER_KEY: &str = "variabler";

    let json = match io_utils::parse_json(filinnhold) {
        Ok(json) => json,
        Err(e) => return Err(format!("Klarte ikke parse JSON: {}", e)),
    };

    let mut resultat = ValidationResult::tom();

    let mut definerte_variabler: HashSet<String> = HashSet::new();
    let mut listevariabler: HashSet<String> = HashSet::new();

    let brukte_variabler = finn_brukte_variabler(filinnhold);

    if let Some(variabler) = json.get(VARIABLER_KEY).and_then(Value::as_object) {
        for (key, value) in variabler {
            definerte_variabler.insert(key.to_string());

            match value {
                Value::String(s) => {
                    if s.starts_with('<') && s.ends_with('>') {
                        resultat
                            .udefinerte_variabler
                            .push(format!("{}: {}", key, s));
                    }
                }
                Value::Array(arr) => {
                    listevariabler.insert(key.to_string());
                    if arr.is_empty()
                        || (arr.len() == 1
                            && arr
                                .first()
                                .and_then(Value::as_str)
                                .is_some_and(|s| s.starts_with('<') && s.ends_with('>')))
                    {
                        resultat.udefinerte_variabler.push(format!(
                            "{}: {}",
                            key,
                            serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string())
                        ));
                    }
                }
                _ => {}
            }
        }
    } else {
        return Err("Ingen 'variabler' nøkkel funnet i JSON-filen.".to_string());
    }

    // Sjekker for brukte variabler som ikke er definert
    for var in &brukte_variabler {
        if !definerte_variabler.contains(var) {
            resultat.manglende_variabler.push(var.clone());
        }
    }

    // Sjekker for definerte variabler som ikke er brukt
    for var in &definerte_variabler {
        if !brukte_variabler.contains(var) {
            resultat.ubrukte_variabler.push(var.clone());
        }
    }

    valider_listevariabler(filinnhold, &listevariabler, &mut resultat);
    valider_funksjoner(filinnhold, &mut resultat);
    valider_operasjoner(&json, &mut resultat);
    valider_versjoner(&json, &mut resultat);
    if streng_metainfo {
        valider_metainfo(&json, filsti, &mut resultat);
    }
    valider_grunnlagsdatamapper(&json, &mut resultat);

    sorter_feil(&mut resultat);

    Ok(resultat)
}

/// Gir forutsigbar rekkefølge på utskriften (HashSet er uordnet) og fjerner
/// duplikater i lister som kan få gjentatte oppføringer.
fn sorter_feil(resultat: &mut ValidationResult) {
    resultat.manglende_variabler.sort();
    resultat.ubrukte_variabler.sort();
    resultat.ukjente_funksjoner.sort();
    resultat.listevariabel_feil.sort();
    resultat.ukjente_handlinger.sort();
    resultat.ukjente_handlinger.dedup();
    resultat.strukturfeil.sort();
    resultat.strukturfeil.dedup();
    resultat.grunnlagsdatamappe_feil.sort();
    resultat.grunnlagsdatamappe_feil.dedup();
}

/// Oppdager `${variabel}`-referanser i hele fila (orkestreringa fletter
/// variabler overalt, ikke bare i `operasjoner`).
fn finn_brukte_variabler(s: &str) -> HashSet<String> {
    let mut brukte = HashSet::new();
    let mut start = 0;

    while let Some(pos) = s[start..].find("${") {
        if let Some(slutt) = s[start + pos + 2..].find('}') {
            let var_name = &s[start + pos + 2..start + pos + 2 + slutt];
            brukte.insert(var_name.to_string());
            start += pos + 2 + slutt + 1;
        } else {
            break;
        }
    }
    brukte
}

/// Listevariabler (array) blir kun flettet når de står som egen streng,
/// dvs. `"${variabel}"`. Brukes de inni en annen streng blir de ikke erstattet.
fn valider_listevariabler(
    filinnhold: &str,
    listevariabler: &HashSet<String>,
    resultat: &mut ValidationResult,
) {
    let bytes = filinnhold.as_bytes();
    for variabel in listevariabler {
        let token = format!("${{{}}}", variabel);
        let mut start = 0;
        let mut feilaktig_bruk = false;
        while let Some(pos) = filinnhold[start..].find(&token) {
            let abs = start + pos;
            let før = if abs == 0 { None } else { Some(bytes[abs - 1]) };
            let etter = bytes.get(abs + token.len()).copied();
            if før != Some(b'"') || etter != Some(b'"') {
                feilaktig_bruk = true;
            }
            start = abs + token.len();
        }
        if feilaktig_bruk {
            resultat
                .listevariabel_feil
                .push(format!("\"${{{}}}\"", variabel));
        }
    }
}

/// Finner `#{...}`-funksjonskall og sjekker dem mot katalogen.
fn valider_funksjoner(filinnhold: &str, resultat: &mut ValidationResult) {
    let mut ukjente: HashSet<String> = HashSet::new();
    let mut start = 0;
    while let Some(pos) = filinnhold[start..].find("#{") {
        let innhold_start = start + pos + 2;
        if let Some(slutt) = filinnhold[innhold_start..].find('}') {
            let kall = &filinnhold[innhold_start..innhold_start + slutt];
            if !rutinefil_katalog::er_kjent_funksjon(kall) {
                ukjente.insert(kall.to_string());
            }
            start = innhold_start + slutt + 1;
        } else {
            break;
        }
    }
    resultat.ukjente_funksjoner.extend(ukjente);
}

/// Validerer `operasjoner`-treet: struktur og kjente handlinger.
fn valider_operasjoner(json: &Value, resultat: &mut ValidationResult) {
    match json.get("operasjoner") {
        Some(Value::Array(operasjoner)) => {
            for operasjon in operasjoner {
                valider_operasjon(operasjon, resultat);
            }
        }
        Some(_) => resultat
            .strukturfeil
            .push("'operasjoner' må være en liste.".to_string()),
        None => resultat
            .strukturfeil
            .push("Mangler påkrevd nøkkel 'operasjoner'.".to_string()),
    }
}

fn valider_operasjon(operasjon: &Value, resultat: &mut ValidationResult) {
    let objekt = match operasjon.as_object() {
        Some(obj) => obj,
        None => {
            resultat
                .strukturfeil
                .push("En operasjon er ikke et JSON-objekt.".to_string());
            return;
        }
    };

    // Ignorerte instruksjoner hoppes over av orkestreringa.
    if objekt
        .get("ignorertInstruksjon")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return;
    }

    let handling = match objekt.get("handling").and_then(Value::as_str) {
        Some(h) => h,
        None => {
            resultat
                .strukturfeil
                .push("En operasjon mangler 'handling'.".to_string());
            return;
        }
    };

    match handling {
        "gruppe" => match objekt.get("gruppeAv") {
            Some(Value::Array(barn)) => {
                for b in barn {
                    valider_operasjon(b, resultat);
                }
            }
            _ => resultat
                .strukturfeil
                .push("Handling 'gruppe' mangler liste 'gruppeAv'.".to_string()),
        },
        "forHver" => valider_for_hver(objekt, resultat),
        "deploy_batch" => valider_deploy_batch(objekt, resultat),
        annen => {
            if !rutinefil_katalog::er_kjent_handling(annen) {
                resultat.ukjente_handlinger.push(annen.to_string());
            }
        }
    }
}

fn valider_for_hver(objekt: &serde_json::Map<String, Value>, resultat: &mut ValidationResult) {
    match objekt.get("elementer") {
        Some(Value::Array(elementer)) if !elementer.is_empty() => {}
        Some(Value::Array(_)) => resultat
            .strukturfeil
            .push("Handling 'forHver' har tom liste 'elementer'.".to_string()),
        // I maler er 'elementer' ofte en listevariabel-referanse ("${var}")
        // som blir til en liste først etter variabelfletting.
        Some(Value::String(s)) if er_variabelreferanse(s) => {}
        _ => resultat
            .strukturfeil
            .push("Handling 'forHver' mangler liste 'elementer'.".to_string()),
    }

    match objekt.get("utfør_handlinger") {
        Some(Value::Array(handlinger)) if !handlinger.is_empty() => {
            for h in handlinger {
                valider_operasjon(h, resultat);
            }
        }
        Some(Value::Array(_)) => resultat
            .strukturfeil
            .push("Handling 'forHver' har tom liste 'utfør_handlinger'.".to_string()),
        Some(Value::String(s)) if er_variabelreferanse(s) => {}
        _ => resultat
            .strukturfeil
            .push("Handling 'forHver' mangler liste 'utfør_handlinger'.".to_string()),
    }
}

/// Er verdien en ren variabelreferanse, f.eks. `${avregningsperioder}`?
fn er_variabelreferanse(s: &str) -> bool {
    s.starts_with("${") && s.ends_with('}') && !s[2..].contains("${")
}

fn valider_deploy_batch(objekt: &serde_json::Map<String, Value>, resultat: &mut ValidationResult) {
    match objekt.get("batcher") {
        Some(Value::Array(batcher)) if !batcher.is_empty() => {
            for batch in batcher {
                let mangler_navn = batch.get("navn").and_then(Value::as_str).is_none();
                let mangler_versjon = batch.get("versjon").and_then(Value::as_str).is_none();
                if mangler_navn || mangler_versjon {
                    resultat.strukturfeil.push(
                        "Handling 'deploy_batch' har en batch som mangler 'navn' eller 'versjon'."
                            .to_string(),
                    );
                }
            }
        }
        _ => resultat
            .strukturfeil
            .push("Handling 'deploy_batch' mangler liste 'batcher'.".to_string()),
    }
}

/// Validerer formatet på versjonsverdier som orkestreringa tolker.
fn valider_versjoner(json: &Value, resultat: &mut ValidationResult) {
    sjekk_versjon(json.get("gyldigVersjon"), "gyldigVersjon", resultat);
    if let Some(metainfo) = json.get("metainfo") {
        sjekk_versjon(
            metainfo.get("støttetAvPaOrkBa01FraVersjon"),
            "metainfo.støttetAvPaOrkBa01FraVersjon",
            resultat,
        );
    }
}

fn sjekk_versjon(verdi: Option<&Value>, felt: &str, resultat: &mut ValidationResult) {
    if let Some(Value::String(v)) = verdi {
        // Hopp over verdier som fortsatt er variabler.
        if v.contains("${") {
            return;
        }
        if !rutinefil_katalog::er_gyldig_versjonsformat(v) {
            resultat.versjonsfeil.push(format!("{}: {}", felt, v));
        }
    }
}

/// Sjekker at `metainfo.mal` peker på riktig fil (fanger kopier-lim-feil).
fn valider_metainfo(json: &Value, filsti: &str, resultat: &mut ValidationResult) {
    let mal = json
        .get("metainfo")
        .and_then(|m| m.get("mal"))
        .and_then(Value::as_str);

    if let Some(mal) = mal {
        let mal_navn = basename(mal);
        let fil_navn = basename(filsti);
        if mal_navn != fil_navn {
            resultat.metainfo_feil.push(format!(
                "metainfo.mal peker på '{}', men filen heter '{}'.",
                mal, fil_navn
            ));
        }
    }
}

fn basename(sti: &str) -> &str {
    Path::new(sti)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(sti)
}

/// Sjekker at en batch kjøres mot samme grunnlagsdataMappe som filene ble
/// kopiert til.
///
/// `kopier_fra_arkiv_til_batch` legger filer i mappa `grunnlagsdataMappeTil`
/// for batchen `batchTil`. Når batchen senere kjøres (handling == batch-navn)
/// med en `grunnlagsdataMappe` som peker et annet sted, finner den ikke
/// filene. Tilstanden per batch nullstilles av `rydd_grunnlagsdata_for_batch`,
/// siden det gjøres nye kopieringer etterpå.
fn valider_grunnlagsdatamapper(json: &Value, resultat: &mut ValidationResult) {
    let variabler = hent_variabelverdier(json);
    let mut kopiert_til: HashMap<String, String> = HashMap::new();
    if let Some(Value::Array(operasjoner)) = json.get("operasjoner") {
        for operasjon in operasjoner {
            spor_grunnlagsdatamappe(operasjon, &variabler, &mut kopiert_til, resultat);
        }
    }
}

/// Henter strengverdiene fra `variabler` slik at `${navn}`-referanser kan løses
/// opp til de faktiske mappenavnene før sammenligning.
fn hent_variabelverdier(json: &Value) -> HashMap<String, String> {
    let mut verdier = HashMap::new();
    if let Some(variabler) = json.get("variabler").and_then(Value::as_object) {
        for (navn, verdi) in variabler {
            if let Value::String(s) = verdi {
                verdier.insert(navn.clone(), s.clone());
            }
        }
    }
    verdier
}

/// Løser opp en ren `${navn}`-referanse til verdien sin. Literaler returneres
/// som de er. Udefinerte variabler gir `None` (da hopper vi over sjekken for å
/// unngå falske positive – `manglende_variabler` fanger dem allerede).
fn resolver_mappeverdi(raw: &str, variabler: &HashMap<String, String>) -> Option<String> {
    if er_variabelreferanse(raw) {
        let navn = &raw[2..raw.len() - 1];
        variabler.get(navn).cloned()
    } else {
        Some(raw.to_string())
    }
}

fn spor_grunnlagsdatamappe(
    operasjon: &Value,
    variabler: &HashMap<String, String>,
    kopiert_til: &mut HashMap<String, String>,
    resultat: &mut ValidationResult,
) {
    let objekt = match operasjon.as_object() {
        Some(obj) => obj,
        None => return,
    };

    if objekt
        .get("ignorertInstruksjon")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return;
    }

    let handling = objekt.get("handling").and_then(Value::as_str).unwrap_or("");

    match handling {
        "gruppe" => {
            if let Some(Value::Array(barn)) = objekt.get("gruppeAv") {
                for b in barn {
                    spor_grunnlagsdatamappe(b, variabler, kopiert_til, resultat);
                }
            }
            return;
        }
        "forHver" => {
            if let Some(Value::Array(barn)) = objekt.get("utfør_handlinger") {
                for b in barn {
                    spor_grunnlagsdatamappe(b, variabler, kopiert_til, resultat);
                }
            }
            return;
        }
        "rydd_grunnlagsdata_for_batch" => {
            // Nullstill: etter opprydding gjøres nye kopieringer.
            if let Some(batch) = objekt.get("batch").and_then(Value::as_str) {
                kopiert_til.remove(batch);
            }
            return;
        }
        "kopier_fra_arkiv_til_batch" => {
            let batch = objekt.get("batchTil").and_then(Value::as_str);
            let mappe = objekt.get("grunnlagsdataMappeTil").and_then(Value::as_str);
            if let (Some(batch), Some(mappe)) = (batch, mappe) {
                match resolver_mappeverdi(mappe, variabler) {
                    Some(verdi) => {
                        kopiert_til.insert(batch.to_string(), verdi);
                    }
                    // Ukjent verdi: fjern evt. tidligere tilstand for å unngå
                    // å sammenligne mot utdatert info.
                    None => {
                        kopiert_til.remove(batch);
                    }
                }
            }
            return;
        }
        _ => {}
    }

    // En batch-kjøring: handling == batch-navn og har egen grunnlagsdataMappe.
    if let Some(mappe) = objekt.get("grunnlagsdataMappe").and_then(Value::as_str) {
        if let Some(kopiert) = kopiert_til.get(handling) {
            if let Some(kjort) = resolver_mappeverdi(mappe, variabler) {
                if &kjort != kopiert {
                    resultat.grunnlagsdatamappe_feil.push(format!(
                        "Batch '{}' kjøres med grunnlagsdataMappe '{}', men filene ble kopiert til '{}' (grunnlagsdataMappeTil). Batchen finner ikke filene.",
                        handling, kjort, kopiert
                    ));
                }
            }
        }
    }
}

fn rapporter_valideringsfeil(feilmelding: &str, vars: &[String]) {
    if vars.is_empty() {
        return;
    }
    println!("{}", feilmelding);
    for var in vars {
        println!("   - {}", var);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valider(innhold: &str) -> ValidationResult {
        validering_innhold_rutinefil(innhold, "maler/test/eksempel.json", true).unwrap()
    }

    #[test]
    fn gyldig_fil_gir_ingen_feil() {
        let innhold = r#"{
            "navn": "Test ${versjon}",
            "gyldigVersjon": "0.1.8",
            "metainfo": { "mal": "maler/test/eksempel.json", "støttetAvPaOrkBa01FraVersjon": "0.1.8" },
            "variabler": { "versjon": "1.2.3" },
            "operasjoner": [
                { "handling": "deploy_batch", "batcher": [ { "navn": "pa_res_ba_02", "versjon": "${versjon}" } ] },
                { "handling": "pa_res_ba_02", "uttrekksdato": "01.01.2025" }
            ]
        }"#;
        let r = valider(innhold);
        assert!(
            r.ingen_feil(),
            "forventet ingen feil, fikk struktur: {:?}",
            r.strukturfeil
        );
    }

    #[test]
    fn oppdager_ukjent_handling() {
        let innhold = r#"{
            "navn": "x",
            "variabler": {},
            "operasjoner": [ { "handling": "deploy" } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.ukjente_handlinger, vec!["deploy".to_string()]);
    }

    #[test]
    fn oppdager_ukjent_handling_i_gruppe() {
        let innhold = r#"{
            "navn": "x",
            "variabler": {},
            "operasjoner": [
                { "handling": "gruppe", "gruppeAv": [ { "handling": "finnesikke" } ] }
            ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.ukjente_handlinger, vec!["finnesikke".to_string()]);
    }

    #[test]
    fn oppdager_ukjent_funksjon() {
        let innhold = r##"{
            "navn": "x",
            "variabler": {},
            "operasjoner": [ { "handling": "pa_fak_ba_12", "versjon": "#{sisteBatchVersjon(pa_fak_ba_12)}" } ]
        }"##;
        let r = valider(innhold);
        assert_eq!(
            r.ukjente_funksjoner,
            vec!["sisteBatchVersjon(pa_fak_ba_12)".to_string()]
        );
    }

    #[test]
    fn godtar_kjent_funksjon() {
        let innhold = r##"{
            "navn": "x",
            "variabler": { "id": "#{genererUUID}" },
            "operasjoner": [ { "handling": "pa_res_ba_02", "id": "${id}" } ]
        }"##;
        let r = valider(innhold);
        assert!(r.ukjente_funksjoner.is_empty());
    }

    #[test]
    fn oppdager_manglende_variabel() {
        let innhold = r#"{
            "navn": "Kjører ${dato}",
            "variabler": { "dagsDato": "01.01.2025" },
            "operasjoner": [ { "handling": "pa_res_ba_02", "uttrekksdato": "${dagsDato}" } ]
        }"#;
        let r = valider(innhold);
        assert!(r.manglende_variabler.contains(&"dato".to_string()));
    }

    #[test]
    fn oppdager_strukturfeil_manglende_handling() {
        let innhold = r#"{
            "navn": "x",
            "variabler": {},
            "operasjoner": [ { "uttrekksdato": "01.01.2025" } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.strukturfeil.len(), 1);
    }

    #[test]
    fn oppdager_deploy_batch_uten_versjon() {
        let innhold = r#"{
            "navn": "x",
            "variabler": {},
            "operasjoner": [ { "handling": "deploy_batch", "batcher": [ { "navn": "pa_res_ba_02" } ] } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.strukturfeil.len(), 1);
    }

    #[test]
    fn oppdager_feil_bruk_av_listevariabel() {
        let innhold = r#"{
            "navn": "x",
            "variabler": { "aarskull": ["1990", "1991"] },
            "operasjoner": [ { "handling": "pa_res_ba_02", "filter": "kull-${aarskull}" } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.listevariabel_feil, vec!["\"${aarskull}\"".to_string()]);
    }

    #[test]
    fn godtar_riktig_bruk_av_listevariabel() {
        let innhold = r#"{
            "navn": "x",
            "variabler": { "aarskull": ["1990", "1991"] },
            "operasjoner": [ { "handling": "pa_res_ba_02", "kull": "${aarskull}" } ]
        }"#;
        let r = valider(innhold);
        assert!(r.listevariabel_feil.is_empty());
    }

    #[test]
    fn oppdager_ugyldig_versjon() {
        let innhold = r#"{
            "navn": "x",
            "gyldigVersjon": "x.x.x",
            "variabler": {},
            "operasjoner": [ { "handling": "pa_res_ba_02" } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.versjonsfeil.len(), 1);
    }

    #[test]
    fn oppdager_feil_metainfo_mal() {
        let innhold = r#"{
            "navn": "x",
            "metainfo": { "mal": "maler/test/noe_annet.json" },
            "variabler": {},
            "operasjoner": [ { "handling": "pa_res_ba_02" } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.metainfo_feil.len(), 1);
    }

    #[test]
    fn metainfo_mal_sjekk_er_av_som_standard() {
        // Uten --streng-metainfo skal mismatch mellom filnavn og metainfo.mal
        // ikke flagges (filer blir ofte omdøpt før kjøring).
        let innhold = r#"{
            "navn": "x",
            "metainfo": { "mal": "maler/test/noe_annet.json" },
            "variabler": {},
            "operasjoner": [ { "handling": "pa_res_ba_02" } ]
        }"#;
        let r = validering_innhold_rutinefil(innhold, "test-omdøpt-202608.json", false).unwrap();
        assert!(
            r.metainfo_feil.is_empty(),
            "metainfo.mal skal ikke sjekkes uten --streng-metainfo, fikk: {:?}",
            r.metainfo_feil
        );
    }

    #[test]
    fn utfylt_verdi_som_slutter_med_vinkel_er_ikke_udefinert() {
        // Regresjonstest for OR->AND-fiksen: en ekte verdi som tilfeldigvis
        // slutter på '>' skal ikke flagges som uutfylt.
        let innhold = r#"{
            "navn": "x",
            "variabler": { "melding": "a > b" },
            "operasjoner": [ { "handling": "pa_res_ba_02", "m": "${melding}" } ]
        }"#;
        let r = valider(innhold);
        assert!(r.udefinerte_variabler.is_empty());
    }

    #[test]
    fn godtar_for_hver_med_listevariabel_referanse() {
        // 'elementer' er ofte en listevariabel-referanse i maler.
        let innhold = r#"{
            "navn": "x",
            "variabler": { "perioder": ["<p>"] },
            "operasjoner": [
                { "handling": "forHver", "elementer": "${perioder}",
                  "utfør_handlinger": [ { "handling": "pa_res_ba_02", "p": "{{element}}" } ] }
            ]
        }"#;
        let r = valider(innhold);
        assert!(
            r.strukturfeil.is_empty(),
            "forventet ingen strukturfeil, fikk: {:?}",
            r.strukturfeil
        );
        assert!(r.listevariabel_feil.is_empty());
    }

    #[test]
    fn oppdager_uutfylt_placeholder() {
        let innhold = r#"{
            "navn": "x",
            "variabler": { "versjon": "<x.x.x>" },
            "operasjoner": [ { "handling": "pa_res_ba_02", "v": "${versjon}" } ]
        }"#;
        let r = valider(innhold);
        assert_eq!(r.udefinerte_variabler.len(), 1);
    }

    #[test]
    fn oppdager_feil_grunnlagsdatamappe() {
        // Filer kopieres til -32, men batchen kjøres mot -82.
        let innhold = r#"{
            "navn": "x",
            "variabler": {
                "mappeTil": "grunnlagsdata_2026-10-01_13-07-34-32",
                "mappe": "grunnlagsdata_2026-10-01_13-07-34-82"
            },
            "operasjoner": [
                { "handling": "gruppe", "gruppeAv": [
                    { "handling": "rydd_grunnlagsdata_for_batch", "batch": "pa_fak_ba_09" },
                    { "handling": "kopier_fra_arkiv_til_batch", "batchTil": "pa_fak_ba_09",
                      "grunnlagsdataMappeTil": "${mappeTil}" },
                    { "handling": "pa_fak_ba_09", "grunnlagsdataMappe": "${mappe}" }
                ] }
            ]
        }"#;
        let r = valider(innhold);
        assert_eq!(
            r.grunnlagsdatamappe_feil.len(),
            1,
            "forventet én grunnlagsdatamappe-feil, fikk: {:?}",
            r.grunnlagsdatamappe_feil
        );
    }

    #[test]
    fn godtar_lik_grunnlagsdatamappe() {
        // Samme mappe brukes ved kopiering og kjøring.
        let innhold = r#"{
            "navn": "x",
            "variabler": { "mappe": "grunnlagsdata_2026-10-01_13-07-34-32" },
            "operasjoner": [
                { "handling": "gruppe", "gruppeAv": [
                    { "handling": "kopier_fra_arkiv_til_batch", "batchTil": "pa_fak_ba_09",
                      "grunnlagsdataMappeTil": "${mappe}" },
                    { "handling": "pa_fak_ba_09", "grunnlagsdataMappe": "${mappe}" }
                ] }
            ]
        }"#;
        let r = valider(innhold);
        assert!(
            r.grunnlagsdatamappe_feil.is_empty(),
            "forventet ingen feil, fikk: {:?}",
            r.grunnlagsdatamappe_feil
        );
    }

    #[test]
    fn nullstiller_grunnlagsdatamappe_etter_rydd() {
        // Etter rydd_grunnlagsdata_for_batch er tidligere kopiering ikke lenger
        // relevant – en påfølgende kjøring mot en annen mappe skal ikke flagges.
        let innhold = r#"{
            "navn": "x",
            "variabler": {
                "mappeA": "grunnlagsdata_A",
                "mappeB": "grunnlagsdata_B"
            },
            "operasjoner": [
                { "handling": "gruppe", "gruppeAv": [
                    { "handling": "kopier_fra_arkiv_til_batch", "batchTil": "pa_fak_ba_09",
                      "grunnlagsdataMappeTil": "${mappeA}" },
                    { "handling": "rydd_grunnlagsdata_for_batch", "batch": "pa_fak_ba_09" },
                    { "handling": "pa_fak_ba_09", "grunnlagsdataMappe": "${mappeB}" }
                ] }
            ]
        }"#;
        let r = valider(innhold);
        assert!(
            r.grunnlagsdatamappe_feil.is_empty(),
            "forventet ingen feil etter rydd, fikk: {:?}",
            r.grunnlagsdatamappe_feil
        );
    }
}
