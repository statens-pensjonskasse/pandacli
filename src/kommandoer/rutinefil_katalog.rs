//! Versjonert katalog over hva `panda-orkestrering` faktisk støtter.
//!
//! Dette er en håndvedlikeholdt («approach A») kopi av de gyldige handlingene og
//! funksjonene i konsumentsystemet `panda-orkestrering`. Katalogen brukes av
//! `valider`-kommandoen til å oppdage obskure feil (ukjente handlinger, ukjente
//! `#{...}`-funksjoner) før rutinefila kjøres.
//!
//! NB! Nye instruksjoner dukker sjelden opp. Når de gjør det, må denne fila og
//! `docs/rutinefil_katalog.md` oppdateres samtidig. Se dokumentet for hvordan du
//! henter verdiene fra `panda-orkestrering`.

/// Versjonen av `panda-orkestrering` denne katalogen sist ble synkronisert mot.
/// Oppdater sammen med innholdet under når katalogen endres.
pub const KATALOG_SYNKET_MOT_ORKESTRERING_VERSJON: &str = "0.1.8";

/// Spesielle handlinger som tolkes direkte av orkestreringa (ikke via en
/// `Oversetter`): gruppering og løkker.
pub const SPESIELLE_HANDLINGER: &[&str] = &["gruppe", "forHver"];

/// Alle `handling`-verdier som har en `Oversetter` i `DelegerOversetting`.
/// Hentet fra `Oversetter.navn()` i `panda-orkestrering`.
pub const KJENTE_HANDLINGER: &[&str] = &[
    "arkiver_all_batch_output",
    "arkiver_batch_output",
    "arkiver_reserveberegning_grunnlagsdata_fra_aktuar",
    "arkiver_termininfofil_fra_aktuar",
    "arkiver_utdata_pa_pre_ba_01",
    "arkiver_utdata_pa_res_ba_02",
    "bakoverkompatibel_batchrapportering_https",
    "deploy_batch",
    "erstatt_isolert_pensjonsvedtak",
    "hent_spesifisert_del_av_medlemsdata",
    "konkatiner_og_kopier_fil_fra_aktuar_til_batch",
    "kopier_arkivert_grunnlagsdata_fra_aktuar_til_pa_res_ba_01",
    "kopier_arkivert_pa_res_ba_02_til_pa_res_ba_01",
    "kopier_arkivert_pa_res_ba_02_til_pa_res_ba_03",
    "kopier_fil_fra_arkiv_aktuar_til_batch",
    "kopier_fil_fra_arkiv_fakturering_til_batch",
    "kopier_filer_fra_aktuar_til_batch",
    "kopier_filer_fra_aktuar_til_skyopplaster",
    "kopier_filer_fra_arkiv_til_skyopplaster",
    "kopier_filer_fra_batch_til_batch",
    "kopier_filer_fra_batch_til_skyopplaster",
    "kopier_filer_fra_pa_pre_ba_01_til_pa_res_ba_01",
    "kopier_fra_arkiv_til_batch",
    "kopier_fra_mellomlager_til_batch",
    "kopier_grunnlagsdata_fra_arkiv_til_batch",
    "kopier_grunnlagsdata_fra_pa_res_ba_02_til_batch",
    "kopier_pa_ork_ba_01_til_pa_ork_ba_05",
    "kopier_termininfofil_fra_arkiv_til_batch",
    "last_ned_referanse_data_for_batch",
    "legg_kontofoering_i_medlemsdata_for_manipulering",
    "mellomlagre_all_batch_output",
    "opprett_bestilling_til_skyopplaster",
    "opprett_filliste_og_triggerfil_hos_dvh",
    "pa_dok_ba_01",
    "pa_dok_ba_02",
    "pa_fak_ba_02",
    "pa_fak_ba_03",
    "pa_fak_ba_04",
    "pa_fak_ba_05",
    "pa_fak_ba_06",
    "pa_fak_ba_07",
    "pa_fak_ba_08",
    "pa_fak_ba_09",
    "pa_fak_ba_10",
    "pa_fak_ba_11",
    "pa_fak_ba_12",
    "pa_fak_ba_13",
    "pa_fak_ba_14",
    "pa_fak_ba_15",
    "pa_ork_ba_02",
    "pa_ork_ba_05",
    "pa_pre_ba_01",
    "pa_pre_ba_02",
    "pa_res_ba_01",
    "pa_res_ba_02",
    "pa_res_ba_03",
    "regelverk_til_pa_res_ba_01",
    "rydd_filområde_hos_dvh",
    "rydd_grunnlagsdata_for_batch",
    "rydd_mellomlager",
    "rydd_utmappe_for_batch",
    "send_bare_filer_fra_arkiv_til_dvh",
    "send_bare_filer_fra_batch_output_til_dvh",
    "send_batch_output_til_dvh",
    "send_til_agresso",
    "slett_foreldede_filer_arkiv",
];

/// `#{...}`-funksjoner som matches på nøyaktig navn i `TekstTilFunksjonHjelper`.
pub const KJENTE_FUNKSJONER_EKSAKT: &[&str] =
    &["grunnlagsdatamappeNå", "genererUUID", "gyldigTermininfo"];

/// `#{...}`-funksjoner som matches på prefiks (de tar argumenter), f.eks.
/// `finnArkivertGrunnlagsdataMappeFraPaResBa02(<yyyy.MM.dd>)`.
pub const KJENTE_FUNKSJONER_PREFIKS: &[&str] = &[
    "finnArkivertGrunnlagsdataMappeFraPaResBa02",
    "finnLevendeGrunnlagsdataMappeFraPaResBa02",
];

/// Er dette en gyldig `handling`-verdi (spesiell eller kjent oversetter)?
pub fn er_kjent_handling(handling: &str) -> bool {
    SPESIELLE_HANDLINGER.contains(&handling) || KJENTE_HANDLINGER.contains(&handling)
}

/// Er dette kallet til en `#{...}`-funksjon støttet av konsumentsystemet?
/// `kall` er innholdet mellom `#{` og `}`, f.eks. `genererUUID` eller
/// `finnArkivertGrunnlagsdataMappeFraPaResBa02(01.01.2025)`.
pub fn er_kjent_funksjon(kall: &str) -> bool {
    let navn = funksjonsnavn(kall);
    KJENTE_FUNKSJONER_EKSAKT.contains(&navn)
        || KJENTE_FUNKSJONER_PREFIKS
            .iter()
            .any(|prefiks| kall.starts_with(prefiks))
}

/// Trekker ut funksjonsnavnet (delen før en eventuell parentes).
fn funksjonsnavn(kall: &str) -> &str {
    match kall.find('(') {
        Some(pos) => &kall[..pos],
        None => kall,
    }
}

/// Validerer formatet på en versjonsstreng, tilsvarende `Versjon`-klassen i
/// `panda-orkestrering`: `\d+(\.\d+)*(-SNAPSHOT)?`.
pub fn er_gyldig_versjonsformat(versjon: &str) -> bool {
    let kjerne = versjon.strip_suffix("-SNAPSHOT").unwrap_or(versjon);
    if kjerne.is_empty() {
        return false;
    }
    kjerne
        .split('.')
        .all(|ledd| !ledd.is_empty() && ledd.bytes().all(|b| b.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kjenner_igjen_spesielle_og_vanlige_handlinger() {
        assert!(er_kjent_handling("gruppe"));
        assert!(er_kjent_handling("forHver"));
        assert!(er_kjent_handling("deploy_batch"));
        assert!(er_kjent_handling("pa_res_ba_02"));
        assert!(er_kjent_handling("pa_pre_ba_01"));
    }

    #[test]
    fn avviser_ukjent_handling() {
        assert!(!er_kjent_handling("deploy"));
        assert!(!er_kjent_handling("pa_res_ba_99"));
        assert!(!er_kjent_handling(""));
    }

    #[test]
    fn kjenner_igjen_eksakte_funksjoner() {
        assert!(er_kjent_funksjon("genererUUID"));
        assert!(er_kjent_funksjon("grunnlagsdatamappeNå"));
        assert!(er_kjent_funksjon("gyldigTermininfo"));
    }

    #[test]
    fn kjenner_igjen_prefiks_funksjoner_med_argument() {
        assert!(er_kjent_funksjon(
            "finnArkivertGrunnlagsdataMappeFraPaResBa02(01.01.2025)"
        ));
        assert!(er_kjent_funksjon(
            "finnLevendeGrunnlagsdataMappeFraPaResBa02(2025.01.01)"
        ));
    }

    #[test]
    fn avviser_ukjent_funksjon() {
        assert!(!er_kjent_funksjon("sisteBatchVersjon(pa_fak_ba_12)"));
        assert!(!er_kjent_funksjon("tullefunksjon"));
    }

    #[test]
    fn validerer_versjonsformat() {
        assert!(er_gyldig_versjonsformat("0.1.8"));
        assert!(er_gyldig_versjonsformat("1"));
        assert!(er_gyldig_versjonsformat("10.20.30"));
        assert!(er_gyldig_versjonsformat("1.2.3-SNAPSHOT"));
    }

    #[test]
    fn avviser_ugyldig_versjonsformat() {
        assert!(!er_gyldig_versjonsformat("x.x.x"));
        assert!(!er_gyldig_versjonsformat("<x.x.x>"));
        assert!(!er_gyldig_versjonsformat("1..2"));
        assert!(!er_gyldig_versjonsformat(""));
        assert!(!er_gyldig_versjonsformat("1.2.3-RELEASE"));
    }
}
