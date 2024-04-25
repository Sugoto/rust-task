use std::fs::File;
use std::io::{self, BufRead};

fn count_lines(text: &str) -> (usize, usize, usize) {
    let mut total_lines = 0;
    let mut code_lines = 0;
    let mut empty_lines = 0;

    for line in text.lines() {
        total_lines += 1;
        if line.trim().is_empty() {
            empty_lines += 1;
        } else {
            code_lines += 1;
        }
    }

    (total_lines, code_lines, empty_lines)
}

fn main() {
    let file_path = "CODE_FRAGMENT.txt";

    let mut input_text = String::new();

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Unable to open the file '{}'", file_path);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        input_text += &line.unwrap();
        input_text.push('\n');
    }

    let (total_lines, code_lines, empty_lines) = count_lines(&input_text);

    println!("Lines in total: {}", total_lines);
    println!("Lines containing code: {}", code_lines);
    println!("Empty lines: {}", empty_lines);
}
