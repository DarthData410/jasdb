// src/test_header.rs
use jasdb::header::*;
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    let path = Path::new("test.jasdb");

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .expect("Failed to open file");

    write_header(&mut file).expect("Failed to write header");
    let (start, end) = read_header(&mut file).expect("Failed to read header");

    println!("✅ Header written and verified.");
    println!("TOC Start Offset: {}", start);
    println!("TOC End Offset: {}", end);
}
