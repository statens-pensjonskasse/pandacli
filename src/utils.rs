use num_format::{Locale, ToFormattedString};


pub fn formater_tall(number: f64) -> String {

    let avrunding = format!("{:.2}", number);
    let splittet_tall: Vec<&str> = avrunding.split('.').collect();
    let heltall = splittet_tall[0].parse::<i64>().unwrap();
    let desimaler = splittet_tall[1];

    let formatted_int = heltall
        .to_formatted_string(&Locale::fr)
        .replace('\u{202f}', " ");

    format!("{}.{}", formatted_int, desimaler)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_store_tall() {
        let n = 4_121_244_855_944.870004;
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
}
