pub mod pdf;
pub mod docx;
pub mod pptx;
pub mod xlsx;
pub mod txt;

pub fn extract_text_from_file(file: String) -> String {
    println!("Extracting text from file: {}", file);
    if file.ends_with(".pdf") {
        pdf::extract(&file)
    } else if file.ends_with(".txt") || file.ends_with(".md") {
        txt::extract(&file)
    } else if file.ends_with(".docx") {
        docx::extract(&file)
    } else if file.ends_with(".pptx") {
        pptx::extract(&file)
    } else if file.ends_with(".xlsx") {
        xlsx::extract(&file)
    } else {
        String::new()
    }
}