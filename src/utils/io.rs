use std::fs;

use inquire::Text;

pub fn prompt_for_file() -> String {
    fs::read_to_string(Text::new("Path:").prompt().unwrap()).expect("File not found.")
}
