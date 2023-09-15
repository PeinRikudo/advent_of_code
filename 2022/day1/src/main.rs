use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = read_file_from_args(&args);

    let mut max_calories = content
        .split("\r\n\r\n")
        .map(|e| e.lines().map(|e| e.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    max_calories.sort_unstable();

    println!("{:?}", max_calories.into_iter().rev().take(1).sum::<i32>());
}

fn read_file_from_args(args: &[String]) -> String {
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).unwrap();

    content
}
