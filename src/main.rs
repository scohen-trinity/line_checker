use std::{fs, env};
// use std::io::{self, Read};

fn main() {
    println!("Line Checker V 0.1.0");
    let env_args: Vec<String> = env::args().collect();

    let file_path: &str = &env_args[1];

    let line_count: usize = check_file(file_path);

    println!("{} lines in file {}", line_count, file_path);
}

fn check_file(file_path: &str) -> usize {
    let imported: String = fs::read_to_string(file_path).expect("Could not read file.");
    
    let lines: Vec<&str> = imported.lines().filter(|&line| !line.starts_with("//") && !line.trim().is_empty()).collect();

    let line_count: usize = lines.len();

    line_count
}