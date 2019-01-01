use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs::read_to_string;

pub fn read_lines_from_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(&file);
    let mut result = Vec::new();
    for line in file.lines() {
        result.push(line.unwrap());
    }
    return result;
}

pub fn read_file(file_name: &str) -> String {
    read_to_string(file_name).expect("Error during reading a file")
}