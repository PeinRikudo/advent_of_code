use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).unwrap();
    println!("The file: {}", file_path);
    println!("Contain: {}", content);
}
