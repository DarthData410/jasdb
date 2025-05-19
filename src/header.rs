// src/header.rs

use std::fs::File;
use crate::io::{read_at, write_at};
use crate::utils::debug;

// Database format constants
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PREFIX: &str = env!("PREFIX");
const MAGIC_SIZE: usize = 14;  // Size of the magic string (PREFIX + VERSION)
const U64_SIZE: usize = 8;     // Size of a u64 in bytes
pub const HEADER_SIZE: usize = MAGIC_SIZE + (U64_SIZE * 2);

// Verify our size assumptions at compile time
const _: () = assert!(MAGIC_SIZE == 14, "Magic size must be 14 bytes");
const _: () = assert!(HEADER_SIZE == 30, "Header size must be 30 bytes");

/// Represents the database file header structure
#[derive(Debug)]
pub struct Header {
    /// Magic bytes identifying the file format and version
    pub magic: [u8; MAGIC_SIZE],
    /// Starting position of the table of contents
    pub toc_start: u64,
    /// Ending position of the table of contents
    pub toc_end: u64,
}

/// Creates the magic byte sequence for the header
pub fn get_header_magic() -> [u8; MAGIC_SIZE] {
    let mut magic = [0u8; MAGIC_SIZE];
    let s = format!("{}{}", PREFIX, VERSION);
    let bytes = s.as_bytes();
    // Ensure we don't exceed the magic size
    debug_assert!(bytes.len() == MAGIC_SIZE, 
        "Magic string length must be exactly {} bytes", MAGIC_SIZE);
    magic[..bytes.len()].copy_from_slice(bytes);
    magic
}

/// Writes the header to the beginning of the file
pub fn write_header(file: &mut File, toc_start: u64, toc_end: u64) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(HEADER_SIZE);
    buf.extend_from_slice(&get_header_magic());
    buf.extend_from_slice(&toc_start.to_le_bytes());
    buf.extend_from_slice(&toc_end.to_le_bytes());
    
    write_at(file, 0, &buf)?;
    debug("✅ Header written successfully");
    Ok(())
}

/// Reads and validates the header from the file
pub fn read_header(file: &mut File) -> std::io::Result<Header> {
    let buf = read_at(file, 0, HEADER_SIZE)?;
    let magic = get_header_magic();
    
    if &buf[0..MAGIC_SIZE] != magic {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid magic header: file format not recognized"
        ));
    }

    let toc_start = u64::from_le_bytes(buf[MAGIC_SIZE..MAGIC_SIZE + U64_SIZE]
        .try_into()
        .expect("Invalid TOC start bytes"));
    let toc_end = u64::from_le_bytes(buf[MAGIC_SIZE + U64_SIZE..HEADER_SIZE]
        .try_into()
        .expect("Invalid TOC end bytes"));

    Ok(Header {
        magic,
        toc_start,
        toc_end,
    })
}
