use std::path::Path;

pub fn filen_ikke_er_json(filsti: &str) -> bool {
    let path = Path::new(filsti);
    if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
        eprintln!("🚫 Error: Filen '{}' har ikke .json filending.", filsti);
        return true;
    }
    false
}