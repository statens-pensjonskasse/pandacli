/// Formaterer et tall med to desimaler og mellomrom som tusenskille,
/// f.eks. `-1234.56` -> `"-1 234.56"`.
///
/// Grupperer sifrene direkte på strengen i stedet for å parse til heltall, slik
/// at vilkårlig store tall fungerer. NaN og uendelig returneres uformatert.
pub fn formater_tall(number: f64) -> String {
    if !number.is_finite() {
        return number.to_string();
    }

    let avrunding = format!("{:.2}", number);
    let (fortegn, uten_fortegn) = match avrunding.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", avrunding.as_str()),
    };
    let (heltall, desimaler) = uten_fortegn
        .split_once('.')
        .unwrap_or((uten_fortegn, "00"));

    let mut gruppert = String::with_capacity(heltall.len() + heltall.len() / 3);
    for (i, siffer) in heltall.chars().enumerate() {
        if i > 0 && (heltall.len() - i) % 3 == 0 {
            gruppert.push(' ');
        }
        gruppert.push(siffer);
    }

    // Små negative tall som rundes til null skal ikke vises som "-0.00".
    let fortegn = if heltall.bytes().all(|b| b == b'0') && desimaler.bytes().all(|b| b == b'0') {
        ""
    } else {
        fortegn
    };

    format!("{}{}.{}", fortegn, gruppert, desimaler)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_store_tall() {
        let n = 4_121_244_855_944.87;
        assert_eq!(formater_tall(n), "4 121 244 855 944.87");
    }

    #[test]
    fn avrunder_korrekt() {
        let n = 1234.999;
        assert_eq!(formater_tall(n), "1 235.00");
    }

    #[test]
    fn håndterer_små_tall() {
        let n = 0.3;
        assert_eq!(formater_tall(n), "0.30");
    }

    #[test]
    fn håndtererer_null_beløp() {
        assert_eq!(formater_tall(0.0), "0.00");
    }

    #[test]
    fn håndterer_negative_tall() {
        assert_eq!(formater_tall(-1234.56), "-1 234.56");
    }

    #[test]
    fn håndterer_tall_større_enn_i64() {
        // Panikket tidligere fordi heltallsdelen ble parset som i64.
        assert_eq!(formater_tall(1e19), "10 000 000 000 000 000 000.00");
    }

    #[test]
    fn håndterer_nan_og_uendelig() {
        assert_eq!(formater_tall(f64::NAN), "NaN");
        assert_eq!(formater_tall(f64::INFINITY), "inf");
        assert_eq!(formater_tall(f64::NEG_INFINITY), "-inf");
    }

    #[test]
    fn grupperer_på_tusengrenser() {
        assert_eq!(formater_tall(999.0), "999.00");
        assert_eq!(formater_tall(1000.0), "1 000.00");
        assert_eq!(formater_tall(100_000.0), "100 000.00");
        assert_eq!(formater_tall(1_000_000.0), "1 000 000.00");
    }

    #[test]
    fn negativt_tall_som_rundes_til_null_mister_fortegn() {
        assert_eq!(formater_tall(-0.001), "0.00");
    }
}
