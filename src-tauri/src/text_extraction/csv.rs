// Extract text from a .csv file
use std::error::Error;

pub fn extract(file: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
    let file = std::fs::read_to_string(file)?;
    let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();
    let mut text = lines.join("\n");
    text = text.chars().filter(|c| !c.is_numeric()).collect();
    // keep only unique words
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut unique_words = Vec::new();
    for word in words {
        if !unique_words.contains(&word) {
        unique_words.push(word);
        }
    }
    Ok(text)
}