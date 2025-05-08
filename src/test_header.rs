use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use jasdb::header::{write_header, read_header, HEADER_MAGIC};

fn main() {
    let path = Path::new("test_header.jasdb");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("❌ Failed to open file");

    // Test writing a header with dummy TOC offsets
    let toc_start = 1024;
    let toc_end = 2048;

    write_header(&mut file, toc_start, toc_end)
        .expect("❌ Failed to write header");

    // Rewind file to test reading
    file.seek(SeekFrom::Start(0)).expect("❌ Seek failed");

    let header = read_header(&mut file)
        .expect("❌ Failed to read header");

    println!("✅ Magic Header: {:?}", String::from_utf8_lossy(&header.magic));
    println!("📌 TOC Start: {}", header.toc_start);
    println!("📌 TOC End: {}", header.toc_end);

    // Optional: Verify the data matches what we wrote
    assert_eq!(&header.magic, HEADER_MAGIC);
    assert_eq!(header.toc_start, toc_start);
    assert_eq!(header.toc_end, toc_end);

    println!("✅ Header integrity check passed");
}
