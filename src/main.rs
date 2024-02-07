use std::fs;
// use std::io::{self, Read};

fn main() {
    println!("Line Checker V 0.1.0");
    let imported: String = import_file("something.txt");
    println!("{imported}");
}

fn import_file(file_path: &str) -> String {
    println!("{}", file_path);

    let read_file = fs::read_to_string(file_path).expect("Could not read file");

    read_file
}