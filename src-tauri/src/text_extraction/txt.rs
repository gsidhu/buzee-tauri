// Extract text from a .txt or .md file
use std::error::Error;
use std::fs::{File, read_to_string, metadata};
use std::io::{prelude::*, BufReader};

pub fn extract(filepath: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
    // read filesize
    let filesize = metadata(filepath.clone())?.len();
    let text: String;
    // if the file is greater than 1MB, read the buffer to extract the text
    if filesize > 1000000 {
        text = extract_large_file(&filepath).unwrap();
    } else {
        let file_contents = read_to_string(filepath)?;
        let lines: Vec<String> = file_contents.lines().map(|line| line.to_string()).collect();
        text = lines.join("\n");
    }
    Ok(text)
}

pub fn extract_large_file(filepath: &String) -> Result<String, Box<dyn Error>> {
    let file_buffer = File::open(filepath)?;
    let reader = BufReader::new(file_buffer);
    let text = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>().join("\n");
    Ok(text)
}