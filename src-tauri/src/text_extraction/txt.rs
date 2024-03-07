// Extract text from a .txt file

pub fn extract(file: &String) -> String {
    let file = std::fs::read_to_string(file).unwrap();
    let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();
    let text = lines.join("\n");
    // println!("Text: {}", lines[0]);
    text
}