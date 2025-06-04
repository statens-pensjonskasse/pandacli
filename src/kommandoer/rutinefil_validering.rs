use std::fs;

pub fn handle_verify_file(file_path: &str) {
    println!("Verifying file: {}", file_path);

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(_) => println!("File is valid JSON ✅"),
        Err(e) => eprintln!("Invalid JSON ❌: {}", e),
    }
}