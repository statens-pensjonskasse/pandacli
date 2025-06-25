use std::path::Path;

pub fn filen_er_ikke_json(filsti: &str) -> bool {
    let path = Path::new(filsti);
    path.extension().and_then(|ext| ext.to_str()) != Some("json")
}

pub fn les_filinnhold(filsti: &str) -> Result<String, String> {
    if filen_er_ikke_json(filsti) {
        return Err(format!("Filen '{}' er ikke en gyldig JSON-fil.", filsti));
    }

    match std::fs::read_to_string(filsti) {
        Ok(innhold) => Ok(innhold),
        Err(e) => Err(format!("Klarte ikke lese filen '{}': {}", filsti, e)),
    }
}

pub fn parse_json(filinnhold: &str) -> Result<serde_json::Value, String> {
    match serde_json::from_str(&filinnhold) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!(
            "Klarte ikke parse JSON fra filen: {}",
            e
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn midlertidig_fil(filename: &str) -> PathBuf {
        let mut path = env::temp_dir();
        path.push(filename);
        path
    }

    #[test]
    fn test_filen_er_ikke_json_med_feil_filending() {
        assert!(filen_er_ikke_json("data.txt"));
        assert!(filen_er_ikke_json("data.csv"));
        assert!(filen_er_ikke_json("data.xlsx"));
        assert!(filen_er_ikke_json("data.html"));
        assert!(filen_er_ikke_json("data.xml"));
        assert!(filen_er_ikke_json("data.md"));
        assert!(filen_er_ikke_json("data.yaml"));
        assert!(filen_er_ikke_json("data.yml"));
        assert!(filen_er_ikke_json("data.json.kjorer"));
    }

    #[test]
    fn test_filen_er_ikke_json_med_riktig_filending() {
        assert!(!filen_er_ikke_json("data.json"));
    }

    #[test]
    fn test_les_filinnhold_suksess() {
        let path = midlertidig_fil("test.json");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "{{\"key\": \"value\"}}").unwrap();

        let result = les_filinnhold(path.to_str().unwrap());
        assert!(result.is_ok());
        assert!(result.unwrap().contains("key"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_parse_json_korrekt_format() {
        let json = r##"{
    "navn": "value",
    "variabler": {
    "grunnlagsdata": "#{funksjon}",
    "dato": "2025.03.01"
  },
  "operasjoner": [
    {
      "handling": "deploy",
      "batcher": [
        {
          "navn": "navn",
          "versjon": "versjon"
        }
      ]
    }
  ]
}"##;

        let result = parse_json(json);
        assert!(result.is_ok());
    }


    #[test]
    fn test_parse_json_feil_format() {
        let json = r##"{
    "navn": "value",
    "variabler": {
    "grunnlagsdata": "#{funksjon}",
    "dato": "2025.03.01"
  },
  "operasjoner":
    {
      "handling": "deploy",
      "batcher": [
        {
          "navn": "navn",
          "versjon": "versjon"
        }
      ]
    }
  ]
}"##;

        let result = parse_json(json);
        assert!(result.is_err());
    }
}
