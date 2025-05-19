// src/header.rs
// Manages the static header region at the top of every .adb file.

use std::fs::File;
use crate::io::{read_at, write_at};
use crate::utils::debug;

// Pull the crate version at compile time
const VERSION: &str = env!("CARGO_PKG_VERSION");
// Build the magic string
const MAGIC_STR: &str = concat!("ADB0069-v", VERSION);

// The raw bytes we write and check against
pub const HEADER_MAGIC: &[u8] = MAGIC_STR.as_bytes();
// Header size = magic + 2 × u64
pub const HEADER_SIZE: usize = HEADER_MAGIC.len() + 8 + 8;

pub struct Header {
    /// Exactly the bytes of the magic string
    pub magic: Vec<u8>,
    /// Byte offset where the TOC begins
    pub toc_start: u64,
    /// Byte offset where the TOC ends
    pub toc_end: u64,
}

/// Write a new header at the very beginning of the file.
/// Overwrites any existing header.
pub fn write_header(
    file: &mut File,
    toc_start: u64,
    toc_end: u64,
) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(HEADER_SIZE);
    buf.extend_from_slice(HEADER_MAGIC);
    buf.extend_from_slice(&toc_start.to_le_bytes());
    buf.extend_from_slice(&toc_end.to_le_bytes());
    write_at(file, 0, &buf)?;
    debug("✅ Header written");
    Ok(())
}

/// Read and validate the header from the start of the file.
pub fn read_header(file: &mut File) -> std::io::Result<Header> {
    let buf = read_at(file, 0, HEADER_SIZE)?;
    // Check magic
    if buf.len() < HEADER_MAGIC.len() || &buf[0..HEADER_MAGIC.len()] != HEADER_MAGIC {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid magic header",
        ));
    }
    // Parse TOC offsets immediately after the magic
    let start = HEADER_MAGIC.len();
    let toc_start = u64::from_le_bytes(buf[start..start + 8].try_into().unwrap());
    let toc_end = u64::from_le_bytes(buf[start + 8..start + 16].try_into().unwrap());

    Ok(Header {
        magic: HEADER_MAGIC.to_vec(),
        toc_start,
        toc_end,
    })
}
