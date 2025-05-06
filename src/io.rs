// io.rs
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::utils::debug;

/// File format constants
pub const HEADER_MAGIC: &[u8] = b"JASDB01\n";
pub const TOC_RESERVED_SIZE: usize = 1024;
pub const HEADER_MAGIC_LEN: usize = 8;

/// Reads exactly `len` bytes from a given offset.
pub fn read_at(file: &mut File, offset: u64, len: usize) -> io::Result<Vec<u8>> {
    debug(&format!("📥 read_at: offset={}, len={}", offset, len));
    file.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

/// Alias used in db.rs and toc.rs
pub fn read_exact_at(file: &mut File, offset: u64, len: usize) -> io::Result<Vec<u8>> {
    read_at(file, offset, len)
}

/// Writes the given buffer to the file at the specified offset.
pub fn write_at(file: &mut File, offset: u64, buf: &[u8]) -> io::Result<()> {
    debug(&format!("💾 write_at: offset={}, len={}", offset, buf.len()));
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(buf)
}

/// Alias used in db.rs and toc.rs
pub fn write_exact_at(file: &mut File, offset: u64, buf: &[u8]) -> io::Result<()> {
    write_at(file, offset, buf)
}

/// Returns the current EOF offset
pub fn get_eof(file: &mut File) -> io::Result<u64> {
    let end = file.seek(SeekFrom::End(0))?;
    debug(&format!("📏 get_eof: {}", end));
    Ok(end)
}

/// Reads 4 bytes from the current position and returns them as a fixed-size array.
pub fn read_exact_4(file: &mut File) -> io::Result<[u8; 4]> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    debug(&format!("📥 read_exact_4 -> {:?}", buf));
    Ok(buf)
}

/// Writes a 4-byte little-endian u32 to the file at the current position.
pub fn write_u32_le(file: &mut File, value: u32) -> io::Result<()> {
    let bytes = value.to_le_bytes();
    debug(&format!("💾 write_u32_le: {} -> {:?}", value, bytes));
    file.write_all(&bytes)
}

/// Reads a single JSON-encoded document (length-prefixed) from the file.
pub fn read_document(file: &mut File) -> io::Result<Option<Vec<u8>>> {
    match read_exact_4(file) {
        Ok(len_buf) => {
            let len = u32::from_le_bytes(len_buf) as usize;
            debug(&format!("📥 read_document: length={}", len));
            let mut doc_buf = vec![0u8; len];
            file.read_exact(&mut doc_buf)?;
            Ok(Some(doc_buf))
        }
        Err(_) => {
            debug("📥 read_document: EOF or failure");
            Ok(None)
        }
    }
}

/// Writes a JSON document (already serialized as bytes) to the file with length prefix.
pub fn write_document(file: &mut File, doc: &[u8]) -> io::Result<()> {
    let len = doc.len() as u32;
    debug(&format!("💾 write_document: length={}", len));
    write_u32_le(file, len)?;
    file.write_all(doc)
}
