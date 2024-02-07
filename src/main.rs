use std::{fs, env};
// use std::io::{self, Read};

fn main() {
    println!("Line Checker V 0.1.0");
    let env_args: Vec<String> = env::args().collect();

    println!("{:?}", env_args);

    let file_path: &str = &env_args[1];

    let imported: String = import_file(file_path);
    
    let lines: Vec<&str> = imported.lines().collect();

    println!("{:?}", lines);

    let line_count = lines.len();

    println!("{}", line_count);
}

fn import_file(file_path: &str) -> String {

    let read_file = fs::read_to_string(file_path).expect("Could not read file");

    read_file
}