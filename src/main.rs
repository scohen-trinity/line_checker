use std::env;
use line_checker::calculator;
use std::fs;
use std::path;
fn main() {
    println!("Line Checker V 0.1.0");
    let env_args: Vec<String> = env::args().collect();

    let file_path: &path::Path = path::Path::new(&env_args[1]);

    // let line_count: usize = calculator::check_file(file_path, "//");

    // println!("{} lines in file {}", line_count, file_path);

    traverse_folders(file_path);
}

fn traverse_folders(path: &path::Path) -> std::io::Result<()> {
    if path.is_dir() {
        println!("Entering folder: {:?}", path);

        for entry in fs::read_dir(path)? {
            let entry: fs::DirEntry = entry?;
            let entry_path: std::path::PathBuf = entry.path();

           traverse_folders(&entry_path)?;
        }
    } else {
        println!("Found file: {:?}", path.file_name());
    }

    Ok(())
}