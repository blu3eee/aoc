pub mod sort;
pub mod create_image;
use std::fs::{ self, File };

// Read input from a file
pub fn read_input(day_number: usize) -> Vec<String> {
    let path = format!("day{day_number:02}/src/input.txt");
    fs::read_to_string(&path)
        .expect(&format!("{path:?} not available"))
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn create_output_file(day_number: usize) -> File {
    let path = format!("day{day_number:02}/src/output.txt");
    fs::File::create(path).expect("cant create output file")
}
