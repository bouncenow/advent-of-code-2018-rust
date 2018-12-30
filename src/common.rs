use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn read_lines_from_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(&file);
    let mut result = Vec::new();
    for line in file.lines() {
        result.push(line.unwrap());
    }
    return result;
}