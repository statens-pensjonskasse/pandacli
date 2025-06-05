use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub server_url: String,
    pub website_url: String,
}

#[allow(dead_code)]
impl AppConfig {
    fn config_sti() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("pandacli")
            .join("conf.json")
    }

    pub fn last_eller_opprett_config() -> AppConfig {
        let path = Self::config_sti();

        if path.exists() {
            let data = fs::read_to_string(&path).expect("Kunne ikke lese konfigurasjonsfilen");
            serde_json::from_str(&data).expect("Kunne ikke parse konfigurasjonsfilen")
        } else {

            println!("Før kommandolinjeverktøyet brukes for første gang, må det defineres et par ting:");
            let server_url = Self::prompt("Lyn url (inkluder bruker):");
            let website_url = Self::prompt("URL for nexus:");

            let config = AppConfig {
                server_url,
                website_url,
            };

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("Kunne ikke lage konfigurasjonsmappe");
            }

            let json = serde_json::to_string_pretty(&config).unwrap();
            fs::write(&path, json).expect("Kunne ikke lagre konfigurasjonen");

            config
        }
    }

    fn prompt(melding: &str) -> String {
        print!("{}", melding);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}
