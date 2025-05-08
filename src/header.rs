// Manages the static header region at the top of every .adb file.
use std::fs::File;
use crate::io::{read_at, write_at}; 
use crate::utils::debug;

pub const HEADER_MAGIC: &[u8; 14] = b"ADB0069-v0.2.1";
pub const HEADER_SIZE: usize = 30;

pub struct Header {
    pub magic: [u8; 14],
    pub toc_start: u64,
    pub toc_end: u64,
}

pub fn write_header(file: &mut File, toc_start: u64, toc_end: u64) -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(HEADER_SIZE);
    buf.extend_from_slice(HEADER_MAGIC);
    buf.extend_from_slice(&toc_start.to_le_bytes());
    buf.extend_from_slice(&toc_end.to_le_bytes());
    write_at(file, 0, &buf)?;
    debug("✅ Header written");
    Ok(())
}

pub fn read_header(file: &mut File) -> std::io::Result<Header> {
    let buf = read_at(file, 0, HEADER_SIZE)?;
    if &buf[0..14] != HEADER_MAGIC {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid magic header"));
    }

    let toc_start = u64::from_le_bytes(buf[14..22].try_into().unwrap());
    let toc_end   = u64::from_le_bytes(buf[22..30].try_into().unwrap());

    Ok(Header {
        magic: *HEADER_MAGIC,
        toc_start,
        toc_end,
    })
}
