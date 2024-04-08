// Extract text from a .txt or .md file
use std::error::Error;

pub fn extract(file: &String, app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string(file)?;
    let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();
    let text = lines.join("\n");
    Ok(text)
}

pub fn extract_large_file(file: &String) -> Result<String, Box<dyn Error>> {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let text = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>().join("\n");
    Ok(text)
}