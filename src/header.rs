// src/header.rs

use std::fs::File;
use crate::io::{read_at, write_at};
use crate::utils::debug;

// Pull the version number at compile time from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");
// Compose the magic string at runtime
const PREFIX: &str = "ADB0069-v";

pub const HEADER_SIZE: usize = 14 + 8 + 8;

pub struct Header {
    pub magic: [u8; 14],
    pub toc_start: u64,
    pub toc_end: u64,
}

// Helper to get the magic string as a [u8; 14]
pub fn get_header_magic() -> [u8; 14] {
    let mut magic = [0u8; 14];
    let s = format!("{}{}", PREFIX, VERSION); // Should be exactly 14 bytes: 8+1+5
    let bytes = s.as_bytes();
    magic[..bytes.len()].copy_from_slice(bytes);
    magic
}

pub fn write_header(file: &mut File, toc_start: u64, toc_end: u64) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(HEADER_SIZE);
    buf.extend_from_slice(&get_header_magic());
    buf.extend_from_slice(&toc_start.to_le_bytes());
    buf.extend_from_slice(&toc_end.to_le_bytes());
    write_at(file, 0, &buf)?;
    debug("✅ Header written");
    Ok(())
}

pub fn read_header(file: &mut File) -> std::io::Result<Header> {
    let buf = read_at(file, 0, HEADER_SIZE)?;
    let magic = get_header_magic();
    if &buf[0..14] != magic {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid magic header"));
    }
    let toc_start = u64::from_le_bytes(buf[14..22].try_into().unwrap());
    let toc_end = u64::from_le_bytes(buf[22..30].try_into().unwrap());
    Ok(Header {
        magic,
        toc_start,
        toc_end,
    })
}
