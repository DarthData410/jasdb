use std::fs::{File, OpenOptions};
use std::path::Path;
use atolldb::header::{write_header, read_header};
use atolldb::utils::debug;

fn main() {
    let path = Path::new("test_header.adb");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("❌ Failed to open test file");

    write_header(&mut file, 128, 256).expect("❌ Failed to write header");

    let header = read_header(&mut file).expect("❌ Failed to read header");

    debug(&format!("✅ Header Read: MAGIC={:?}, TOC Start={}, TOC End={}", 
        std::str::from_utf8(&header.magic).unwrap(), 
        header.toc_start, 
        header.toc_end));
}
