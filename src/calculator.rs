use std::fs;

pub fn check_file(file_path: &str, comment: &str) -> usize {
    let imported: String = fs::read_to_string(file_path).expect("Could not read file.");
    
    let lines: Vec<&str> = imported.lines().filter(|&line| !line.starts_with(comment) && !line.trim().is_empty()).collect();

    let line_count: usize = lines.len();

    line_count
}