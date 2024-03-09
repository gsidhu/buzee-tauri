// Extract text from a .txt file
use std::error::Error;

pub fn extract(file: &String) -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string(file)?;
    let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();
    let text = lines.join("\n");
    Ok(text)
}